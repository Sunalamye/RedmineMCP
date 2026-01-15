#!/usr/bin/env bun
/**
 * Redmine URL Parser
 * Parse Redmine web URL and output API parameters as JSON
 *
 * Usage: bun run parse-url.ts "<redmine-url>"
 */

interface ApiParams {
  type: "issue" | "issues";
  id?: number;
  project_id?: string;
  status_id?: string;
  tracker_id?: number;
  assigned_to_id?: string;
  limit?: number;
  offset?: number;
  sort?: string;
}

function parseRedmineUrl(urlString: string): ApiParams {
  const url = new URL(urlString);
  const path = url.pathname;

  // Single issue: /issues/{id}
  const issueMatch = path.match(/^\/issues\/(\d+)$/);
  if (issueMatch) {
    return {
      type: "issue",
      id: parseInt(issueMatch[1], 10),
    };
  }

  // Issues list: /projects/{project_id}/issues
  const projectMatch = path.match(/^\/projects\/([^/]+)\/issues$/);
  if (!projectMatch) {
    // Try without project: /issues
    if (path === "/issues") {
      return parseQueryParams(url.searchParams, {
        type: "issues",
      });
    }
    throw new Error(`Unknown URL pattern: ${path}`);
  }

  const params: ApiParams = {
    type: "issues",
    project_id: projectMatch[1],
  };

  return parseQueryParams(url.searchParams, params);
}

function parseQueryParams(
  searchParams: URLSearchParams,
  params: ApiParams
): ApiParams {
  // Collect all parameters
  const filters: string[] = [];
  const operators: Record<string, string> = {};
  const values: Record<string, string[]> = {};

  for (const [key, value] of searchParams) {
    // f[] = filter fields
    if (key === "f[]" && value) {
      filters.push(value);
    }
    // op[field] = operator
    const opMatch = key.match(/^op\[(.+)\]$/);
    if (opMatch) {
      operators[opMatch[1]] = value;
    }
    // v[field][] = values
    const vMatch = key.match(/^v\[(.+)\]\[\]$/);
    if (vMatch) {
      if (!values[vMatch[1]]) {
        values[vMatch[1]] = [];
      }
      values[vMatch[1]].push(value);
    }
  }

  // Parse status_id
  if (operators["status_id"]) {
    const op = operators["status_id"];
    if (op === "o") {
      params.status_id = "open";
    } else if (op === "c") {
      params.status_id = "closed";
    } else if (op === "*") {
      params.status_id = "*";
    } else if (op === "=" && values["status_id"]?.[0]) {
      params.status_id = values["status_id"][0];
    }
  }

  // Parse tracker_id
  if (values["tracker_id"]?.[0]) {
    params.tracker_id = parseInt(values["tracker_id"][0], 10);
  }

  // Parse assigned_to_id
  if (operators["assigned_to_id"]) {
    const op = operators["assigned_to_id"];
    const val = values["assigned_to_id"]?.[0];

    if (op === "=" && val) {
      params.assigned_to_id = val === "me" ? "me" : val;
    } else if (op === "!" && val) {
      params.assigned_to_id = `!${val}`;
    } else if (op === "!*") {
      params.assigned_to_id = "";
    }
  }

  // Parse pagination
  const limit = searchParams.get("limit");
  if (limit) {
    params.limit = parseInt(limit, 10);
  }

  const offset = searchParams.get("offset");
  if (offset) {
    params.offset = parseInt(offset, 10);
  }

  // Parse sort
  const sort = searchParams.get("sort");
  if (sort) {
    params.sort = sort;
  }

  return params;
}

// Main
const url = process.argv[2];

if (!url) {
  console.error("Usage: bun run parse-url.ts <redmine-url>");
  process.exit(1);
}

try {
  const params = parseRedmineUrl(url);
  console.log(JSON.stringify(params, null, 2));
} catch (error) {
  console.error(`Error: ${error}`);
  process.exit(1);
}
