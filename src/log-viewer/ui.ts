/**
 * Log Viewer UI - Single file HTML+CSS+JS
 */

export const LOG_VIEWER_HTML = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>RedmineMCP Log Viewer</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }

    body {
      background: #1a1a2e;
      color: #eaeaea;
      font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
      font-size: 13px;
      line-height: 1.5;
    }

    /* Header */
    .header {
      background: #16213e;
      padding: 12px 20px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      border-bottom: 1px solid #0f3460;
      position: sticky;
      top: 0;
      z-index: 100;
    }

    .header h1 {
      font-size: 16px;
      font-weight: 600;
      color: #e94560;
    }

    .header-actions {
      display: flex;
      gap: 8px;
    }

    .btn {
      background: #0f3460;
      border: 1px solid #e94560;
      color: #eaeaea;
      padding: 6px 12px;
      border-radius: 4px;
      cursor: pointer;
      font-size: 12px;
      transition: all 0.2s;
    }

    .btn:hover { background: #e94560; }
    .btn.active { background: #e94560; }

    /* Toolbar */
    .toolbar {
      background: #16213e;
      padding: 10px 20px;
      display: flex;
      gap: 16px;
      align-items: center;
      flex-wrap: wrap;
      border-bottom: 1px solid #0f3460;
    }

    .toolbar label {
      color: #888;
      font-size: 12px;
    }

    .toolbar select, .toolbar input {
      background: #0f3460;
      border: 1px solid #333;
      color: #eaeaea;
      padding: 6px 10px;
      border-radius: 4px;
      font-size: 12px;
    }

    .toolbar input[type="text"] {
      width: 200px;
    }

    .toolbar select:focus, .toolbar input:focus {
      outline: none;
      border-color: #e94560;
    }

    /* Log Container */
    .log-container {
      padding: 10px 0;
      overflow-y: auto;
      height: calc(100vh - 150px);
    }

    .log-entry {
      padding: 4px 20px;
      display: flex;
      align-items: flex-start;
      gap: 12px;
      border-bottom: 1px solid rgba(255,255,255,0.03);
    }

    .log-entry:hover {
      background: rgba(255,255,255,0.03);
    }

    .log-entry.hidden { display: none; }

    .log-time {
      color: #666;
      white-space: nowrap;
      min-width: 85px;
    }

    .log-level {
      font-weight: 600;
      min-width: 50px;
      text-transform: uppercase;
    }

    .log-level.debug { color: #6b7280; }
    .log-level.info { color: #60a5fa; }
    .log-level.warn { color: #fbbf24; }
    .log-level.error { color: #f87171; }

    .log-tool {
      color: #a78bfa;
      min-width: 180px;
      font-weight: 500;
    }

    .log-duration {
      color: #34d399;
      min-width: 70px;
      width: 70px;
      text-align: right;
      font-variant-numeric: tabular-nums;
    }

    .log-duration.slow { color: #fbbf24; }
    .log-duration.very-slow { color: #f87171; }
    .log-duration.empty { color: #444; }

    .log-message {
      flex: 1;
      word-break: break-word;
      color: #d1d5db;
    }

    /* Clickable JSON */
    .log-entry.has-json {
      cursor: pointer;
    }

    .log-entry.has-json:hover .log-message {
      color: #60a5fa;
    }

    .json-key { color: #f472b6; }
    .json-string { color: #34d399; }
    .json-number { color: #fbbf24; }
    .json-boolean { color: #60a5fa; }
    .json-null { color: #9ca3af; }

    .json-toggle {
      color: #666;
      font-size: 11px;
      margin-left: 8px;
    }

    .log-entry.has-json:hover .json-toggle {
      color: #60a5fa;
    }

    /* JSON Modal */
    .modal-overlay {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0, 0, 0, 0.7);
      display: flex;
      align-items: center;
      justify-content: center;
      z-index: 1000;
      opacity: 0;
      visibility: hidden;
      transition: opacity 0.2s, visibility 0.2s;
    }

    .modal-overlay.visible {
      opacity: 1;
      visibility: visible;
    }

    .modal-content {
      background: #1a1a2e;
      border: 1px solid #0f3460;
      border-radius: 8px;
      width: 90%;
      max-width: 800px;
      max-height: 80vh;
      display: flex;
      flex-direction: column;
      box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
    }

    .modal-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 16px 20px;
      border-bottom: 1px solid #0f3460;
      background: #16213e;
      border-radius: 8px 8px 0 0;
    }

    .modal-title {
      font-size: 14px;
      font-weight: 600;
      color: #e94560;
    }

    .modal-close {
      background: none;
      border: none;
      color: #666;
      font-size: 24px;
      cursor: pointer;
      padding: 0 8px;
      line-height: 1;
    }

    .modal-close:hover {
      color: #e94560;
    }

    .modal-body {
      padding: 20px;
      overflow: auto;
      flex: 1;
    }

    .modal-json {
      background: #0d1b2a;
      border: 1px solid #1e3a5f;
      border-radius: 6px;
      padding: 16px 20px;
      overflow-x: auto;
      font-size: 13px;
      line-height: 1.7;
      white-space: pre-wrap;
    }

    .modal-info {
      font-size: 11px;
      color: #666;
      margin-bottom: 12px;
    }

    /* Status Bar */
    .status-bar {
      background: #16213e;
      padding: 8px 20px;
      display: flex;
      gap: 20px;
      font-size: 11px;
      color: #666;
      border-top: 1px solid #0f3460;
      position: fixed;
      bottom: 0;
      left: 0;
      right: 0;
    }

    .status-indicator {
      display: flex;
      align-items: center;
      gap: 6px;
    }

    .status-dot {
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: #f87171;
    }

    .status-dot.connected { background: #34d399; }

    /* Empty State */
    .empty-state {
      text-align: center;
      padding: 60px 20px;
      color: #666;
    }

    .empty-state h2 {
      font-size: 18px;
      margin-bottom: 10px;
    }
  </style>
</head>
<body>
  <header class="header">
    <h1>RedmineMCP Log Viewer</h1>
    <div class="header-actions">
      <button class="btn" id="pauseBtn">Pause</button>
      <button class="btn" id="clearBtn">Clear</button>
      <button class="btn" id="exportBtn">Export</button>
    </div>
  </header>

  <div class="toolbar">
    <div>
      <label>Level: </label>
      <select id="levelFilter">
        <option value="all">All</option>
        <option value="debug">DEBUG</option>
        <option value="info">INFO</option>
        <option value="warn">WARN</option>
        <option value="error">ERROR</option>
      </select>
    </div>
    <div>
      <label>Tool: </label>
      <select id="toolFilter">
        <option value="all">All</option>
      </select>
    </div>
    <div>
      <label>Search: </label>
      <input type="text" id="searchInput" placeholder="Enter keyword...">
    </div>
    <div>
      <label>
        <input type="checkbox" id="autoScroll" checked> Auto scroll
      </label>
    </div>
  </div>

  <div class="log-container" id="logContainer">
    <div class="empty-state" id="emptyState">
      <h2>Waiting for logs...</h2>
      <p>Logs will appear here when MCP tools are invoked</p>
    </div>
  </div>

  <div class="status-bar">
    <div class="status-indicator">
      <span class="status-dot" id="statusDot"></span>
      <span id="statusText">Connecting...</span>
    </div>
    <div id="entryCount">0 entries</div>
    <div id="lastUpdate">-</div>
  </div>

  <!-- JSON Modal -->
  <div class="modal-overlay" id="jsonModal">
    <div class="modal-content">
      <div class="modal-header">
        <span class="modal-title" id="modalTitle">JSON Details</span>
        <button class="modal-close" id="modalClose">&times;</button>
      </div>
      <div class="modal-body">
        <div class="modal-info" id="modalInfo"></div>
        <div class="modal-json" id="modalJson"></div>
      </div>
    </div>
  </div>

  <script>
    // State
    const state = {
      logs: [],
      paused: false,
      levelFilter: 'all',
      toolFilter: 'all',
      searchQuery: '',
      autoScroll: true,
      connected: false,
      tools: new Set(),
    };

    // DOM Elements
    const logContainer = document.getElementById('logContainer');
    const emptyState = document.getElementById('emptyState');
    const statusDot = document.getElementById('statusDot');
    const statusText = document.getElementById('statusText');
    const entryCount = document.getElementById('entryCount');
    const lastUpdate = document.getElementById('lastUpdate');
    const pauseBtn = document.getElementById('pauseBtn');
    const levelFilter = document.getElementById('levelFilter');
    const toolFilter = document.getElementById('toolFilter');
    const searchInput = document.getElementById('searchInput');
    const autoScrollCheck = document.getElementById('autoScroll');

    // Format time
    function formatTime(timestamp) {
      const date = new Date(timestamp);
      return date.toLocaleTimeString('en-US', { hour12: false }) +
             '.' + String(date.getMilliseconds()).padStart(3, '0');
    }

    // Format duration
    function formatDuration(ms) {
      if (ms === undefined || ms === null) {
        return '<span class="log-duration empty">-</span>';
      }
      let cls = '';
      if (ms > 500) cls = 'slow';
      if (ms > 1000) cls = 'very-slow';
      return '<span class="log-duration ' + cls + '">' + ms + 'ms</span>';
    }

    // Extract JSON from message
    function extractJson(message) {
      // Match JSON object or array in message
      const match = message.match(/(\\{[\\s\\S]*\\}|\\[[\\s\\S]*\\])/);
      if (!match) return null;
      try {
        const parsed = JSON.parse(match[1]);
        return { json: parsed, raw: match[1] };
      } catch {
        return null;
      }
    }

    // Syntax highlight JSON
    function syntaxHighlight(json) {
      const str = JSON.stringify(json, null, 2);
      return str.replace(/("(\\\\u[a-zA-Z0-9]{4}|\\\\[^u]|[^\\\\"])*"(\\s*:)?|\\b(true|false|null)\\b|-?\\d+(?:\\.\\d*)?(?:[eE][+\\-]?\\d+)?)/g, function(match) {
        let cls = 'json-number';
        if (/^"/.test(match)) {
          if (/:$/.test(match)) {
            cls = 'json-key';
            match = match.slice(0, -1); // Remove colon for styling
            return '<span class="' + cls + '">' + escapeHtml(match) + '</span>:';
          } else {
            cls = 'json-string';
          }
        } else if (/true|false/.test(match)) {
          cls = 'json-boolean';
        } else if (/null/.test(match)) {
          cls = 'json-null';
        }
        return '<span class="' + cls + '">' + escapeHtml(match) + '</span>';
      });
    }

    // Modal elements
    const jsonModal = document.getElementById('jsonModal');
    const modalTitle = document.getElementById('modalTitle');
    const modalInfo = document.getElementById('modalInfo');
    const modalJson = document.getElementById('modalJson');
    const modalClose = document.getElementById('modalClose');

    // Show JSON modal
    function showJsonModal(log, jsonData) {
      modalTitle.textContent = log.tool ? log.tool + ' Response' : 'JSON Details';
      modalInfo.textContent = formatTime(log.timestamp) + ' - ' + log.level.toUpperCase();
      modalJson.innerHTML = syntaxHighlight(jsonData.json);
      jsonModal.classList.add('visible');
    }

    // Hide JSON modal
    function hideJsonModal() {
      jsonModal.classList.remove('visible');
    }

    // Modal event listeners
    modalClose.addEventListener('click', hideJsonModal);
    jsonModal.addEventListener('click', function(e) {
      if (e.target === jsonModal) hideJsonModal();
    });
    document.addEventListener('keydown', function(e) {
      if (e.key === 'Escape') hideJsonModal();
    });

    // Create log entry HTML
    function createLogEntry(log) {
      const div = document.createElement('div');
      div.className = 'log-entry';
      div.dataset.level = log.level;
      div.dataset.tool = log.tool || '';
      div.dataset.message = log.message.toLowerCase();

      const jsonData = extractJson(log.message);
      const hasJson = jsonData !== null;

      if (hasJson) {
        div.classList.add('has-json');
      }

      let messageHtml = escapeHtml(log.message);
      if (hasJson) {
        messageHtml += '<span class="json-toggle">[click to view]</span>';
      }

      div.innerHTML =
        '<span class="log-time">' + formatTime(log.timestamp) + '</span>' +
        '<span class="log-level ' + log.level + '">' + log.level + '</span>' +
        '<span class="log-tool">' + (log.tool || '-') + '</span>' +
        formatDuration(log.duration_ms) +
        '<span class="log-message">' + messageHtml + '</span>';

      if (hasJson) {
        div.addEventListener('click', function() {
          showJsonModal(log, jsonData);
        });
      }

      return div;
    }

    // Escape HTML
    function escapeHtml(text) {
      const div = document.createElement('div');
      div.textContent = text;
      return div.innerHTML;
    }

    // Add log entry
    function addLog(log) {
      if (state.paused) {
        state.logs.push(log);
        return;
      }

      state.logs.push(log);

      // Track tools
      if (log.tool && !state.tools.has(log.tool)) {
        state.tools.add(log.tool);
        updateToolFilter();
      }

      // Hide empty state
      emptyState.style.display = 'none';

      // Create and append entry
      const entry = createLogEntry(log);
      applyFilters(entry);
      logContainer.appendChild(entry);

      // Auto scroll
      if (state.autoScroll) {
        entry.scrollIntoView({ behavior: 'smooth', block: 'end' });
      }

      // Update status
      entryCount.textContent = state.logs.length + ' entries';
      lastUpdate.textContent = 'Last: ' + formatTime(log.timestamp);
    }

    // Apply filters to entry
    function applyFilters(entry) {
      const levelMatch = state.levelFilter === 'all' || entry.dataset.level === state.levelFilter;
      const toolMatch = state.toolFilter === 'all' || entry.dataset.tool === state.toolFilter;
      const searchMatch = !state.searchQuery || entry.dataset.message.includes(state.searchQuery.toLowerCase());

      entry.classList.toggle('hidden', !(levelMatch && toolMatch && searchMatch));
    }

    // Reapply filters to all entries
    function reapplyFilters() {
      const entries = logContainer.querySelectorAll('.log-entry');
      entries.forEach(applyFilters);
    }

    // Update tool filter dropdown
    function updateToolFilter() {
      const current = toolFilter.value;
      toolFilter.innerHTML = '<option value="all">All</option>';
      Array.from(state.tools).sort().forEach(tool => {
        const option = document.createElement('option');
        option.value = tool;
        option.textContent = tool;
        toolFilter.appendChild(option);
      });
      toolFilter.value = current;
    }

    // Export logs
    function exportLogs() {
      const text = state.logs.map(log => log.raw).join('\\n');
      const blob = new Blob([text], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'redmine-mcp-logs-' + new Date().toISOString().slice(0,10) + '.txt';
      a.click();
      URL.revokeObjectURL(url);
    }

    // WebSocket connection
    function connect() {
      const ws = new WebSocket('ws://localhost:' + location.port + '/ws');

      ws.onopen = () => {
        state.connected = true;
        statusDot.classList.add('connected');
        statusText.textContent = 'Connected';
      };

      ws.onmessage = (e) => {
        try {
          const log = JSON.parse(e.data);
          addLog(log);
        } catch (err) {
          console.error('Parse error:', err);
        }
      };

      ws.onclose = () => {
        state.connected = false;
        statusDot.classList.remove('connected');
        statusText.textContent = 'Disconnected, reconnecting...';
        setTimeout(connect, 2000);
      };

      ws.onerror = () => {
        ws.close();
      };
    }

    // Event listeners
    pauseBtn.addEventListener('click', () => {
      state.paused = !state.paused;
      pauseBtn.textContent = state.paused ? 'Resume' : 'Pause';
      pauseBtn.classList.toggle('active', state.paused);
    });

    document.getElementById('clearBtn').addEventListener('click', () => {
      state.logs = [];
      state.tools.clear();
      logContainer.innerHTML = '';
      emptyState.style.display = 'block';
      logContainer.appendChild(emptyState);
      updateToolFilter();
      entryCount.textContent = '0 entries';
    });

    document.getElementById('exportBtn').addEventListener('click', exportLogs);

    levelFilter.addEventListener('change', (e) => {
      state.levelFilter = e.target.value;
      reapplyFilters();
    });

    toolFilter.addEventListener('change', (e) => {
      state.toolFilter = e.target.value;
      reapplyFilters();
    });

    searchInput.addEventListener('input', (e) => {
      state.searchQuery = e.target.value;
      reapplyFilters();
    });

    autoScrollCheck.addEventListener('change', (e) => {
      state.autoScroll = e.target.checked;
    });

    // Start
    connect();
  </script>
</body>
</html>`;
