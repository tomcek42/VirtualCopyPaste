/**
 * Settings page controller.
 * Uses tauri-plugin-store for persistence and tauri-plugin-autostart for startup.
 */
(async function () {
  async function waitForTauri(maxMs) {
    var start = Date.now();
    while (!window.__TAURI__ || !window.__TAURI__.store) {
      if (Date.now() - start > maxMs) throw new Error('Tauri API not available');
      await new Promise(function (r) { setTimeout(r, 50); });
    }
  }

  try {
    await waitForTauri(3000);
  } catch (e) {
    document.body.innerHTML = '<p style="color:#d9534f;padding:20px;font-family:sans-serif;">Tauri API not available. Try reopening settings.</p>';
    return;
  }

  var store;
  try {
    store = await Promise.race([
      window.__TAURI__.store.load('settings.json', { autoSave: false }),
      new Promise(function (_, reject) {
        setTimeout(function () { reject(new Error('Store load timeout')); }, 5000);
      })
    ]);
  } catch (err) {
    console.error('Failed to load store:', err);
    document.body.innerHTML = '<p style="color:#d9534f;padding:20px;font-family:sans-serif;">Failed to load settings store: ' + err.message + '</p>';
    return;
  }

  // ── Auto-fit window height ──
  // The window is not resizable, so its height has to carry whatever is currently
  // visible — including an opened Advanced block. Instead of hardcoding the tallest
  // case, measure the content and set the height from it. resizable(false) only blocks
  // the user's own drag handles; setSize still works.
  var MIN_H = 480;
  var fitPending = false;

  function measureNeeded(content, section) {
    // Sum the active section plus the Save button block (a sibling of the sections,
    // not a child) plus the container padding. content.scrollHeight would be the
    // obvious single call, but Chromium leaves the bottom padding out of it once the
    // box overflows, so the last line would be clipped by exactly that much.
    var cs = window.getComputedStyle(content);
    var inner = section.offsetHeight;
    var actions = document.querySelector('.settings-actions');
    if (actions) inner += actions.offsetHeight;
    return Math.ceil(inner + parseFloat(cs.paddingTop) + parseFloat(cs.paddingBottom));
  }

  function fitWindowHeight() {
    // Coalesce the calls that fire together (nav click, details toggle and the mode
    // change that opens the disclosure) into one pass on the next frame, after
    // layout has settled.
    if (fitPending) return;
    fitPending = true;
    requestAnimationFrame(function () {
      fitPending = false;
      var content = document.querySelector('.content');
      var section = document.querySelector('.section.active');
      if (!content || !section) return;
      // Leave room for title bar and taskbar. On a short screen the window would
      // otherwise grow past the desktop with no way to reach the rest — there the
      // clamp hands the job back to .content's own scrollbar.
      var maxH = Math.max(MIN_H, Math.floor(window.screen.availHeight - 80));
      var win;
      try {
        win = window.__TAURI__.window.getCurrentWindow();
      } catch (e) {
        console.error('fitWindowHeight failed:', e);
        return;
      }

      // Re-measure after every resize instead of trusting the first sum. Changing
      // the window height reflows the content (wrapped hint text is the usual
      // culprit), so the height the content needs can itself change with the
      // height it was just given. Iterating settles both directions — the earlier
      // one-shot correction could only ever grow the window, which is why switching
      // Standard -> Compatible with Advanced open landed on the wrong height.
      var passes = 0;
      function pass() {
        var needed = measureNeeded(content, section);
        // .content is overflow-y: hidden by default so no scrollbar can flash while
        // the window is catching up. Only the clamped case — content taller than the
        // screen allows — gets a real scrollbar back, because there is no other way
        // to reach the rest in a window that is not resizable.
        content.style.overflowY = needed > maxH ? 'auto' : 'hidden';
        var target = Math.min(Math.max(needed, MIN_H), maxH);
        var overflow = content.scrollHeight - content.clientHeight;
        if (overflow > 0) target = Math.min(target + Math.ceil(overflow), maxH);
        if (Math.abs(target - window.innerHeight) < 2) return;
        if (++passes > 4) return;
        win.setSize(new window.__TAURI__.window.LogicalSize(480, target));
        requestAnimationFrame(pass);
      }
      pass();
    });
  }

  // ── Sidebar navigation ──
  var navItems = document.querySelectorAll('.nav-item');
  var sections = document.querySelectorAll('.section');

  navItems.forEach(function (item) {
    item.addEventListener('click', function () {
      var target = item.getAttribute('data-section');
      navItems.forEach(function (n) { n.classList.remove('active'); });
      sections.forEach(function (s) { s.classList.remove('active'); });
      item.classList.add('active');
      document.getElementById('section-' + target).classList.add('active');
      fitWindowHeight();
    });
  });

  // ── DOM refs ──
  var activateHotkeyInput = document.getElementById('activateHotkey');
  var clearActivateBtn = document.getElementById('clearActivateHotkey');
  var typingDelaySlider = document.getElementById('typingDelay');
  var typingDelayValue = document.getElementById('typingDelayValue');
  var keyPressDelaySlider = document.getElementById('keyPressDelay');
  var keyPressDelayValue = document.getElementById('keyPressDelayValue');
  var keyboardModeSelect = document.getElementById('keyboardMode');
  var keyboardModeDetails = document.getElementById('keyboardModeDetails');
  var targetLayoutSelect = document.getElementById('targetLayout');
  var targetLayoutGroup = document.getElementById('targetLayoutGroup');
  var clearAutoIndentCb = document.getElementById('clearAutoIndent');
  var inputModeSelect = document.getElementById('inputMode');
  var inputModeDetails = document.getElementById('inputModeDetails');
  var alwaysOnTopCb = document.getElementById('alwaysOnTop');
  var autostartCb = document.getElementById('autostart');
  var startMinimizedCb = document.getElementById('startMinimized');
  var autoCheckUpdatesCb = document.getElementById('autoCheckUpdates');
  var autoClearEnabledCb = document.getElementById('autoClearEnabled');
  var autoClearTimeoutSlider = document.getElementById('autoClearTimeout');
  var autoClearTimeoutValue = document.getElementById('autoClearTimeoutValue');
  var autoClearTimeoutGroup = document.getElementById('autoClearTimeoutGroup');
  var doubleClickEnabledCb = document.getElementById('doubleClickEnabled');
  var autoPasteEnabledCb = document.getElementById('autoPasteEnabled');
  var autoPasteModeSelect = document.getElementById('autoPasteMode');
  var autoPasteModeGroup = document.getElementById('autoPasteModeGroup');
  var saveBtn = document.getElementById('saveSettings');

  // Update UI refs
  var checkUpdatesBtn = document.getElementById('checkUpdates');
  var updateStatusEl = document.getElementById('updateStatus');
  var updateBanner = document.getElementById('updateBanner');
  var updateBannerVersion = document.getElementById('updateBannerVersion');
  var updateInstallBtn = document.getElementById('updateInstallBtn');
  var updateDismissBtn = document.getElementById('updateDismissBtn');
  var updateProgress = document.getElementById('updateProgress');

  // Holds the pending update object from the updater API
  var pendingUpdate = null;
  var currentAppVersion = null;
  try {
    currentAppVersion = await window.__TAURI__.core.invoke('get_version');
  } catch (e) { currentAppVersion = null; }

  function isNewerVersion(remote, current) {
    if (!remote || !current) return false;
    var r = remote.replace(/^v/, '').split('.').map(Number);
    var c = current.replace(/^v/, '').split('.').map(Number);
    for (var i = 0; i < Math.max(r.length, c.length); i++) {
      var rp = r[i] || 0;
      var cp = c[i] || 0;
      if (rp > cp) return true;
      if (rp < cp) return false;
    }
    return false;
  }

  // ── Defaults ──
  var DEFAULTS = {
    activateHotkey: 'Ctrl+Shift+V',
    typingDelay: 20,
    keyPressDelay: 30,
    keyboardMode: 'unicode',
    targetLayout: 'auto',
    clearAutoIndent: true,
    inputMode: 'single',
    alwaysOnTop: false,
    autostart: false,
    startMinimized: false,
    autoCheckUpdates: true,
    autoClearEnabled: false,
    autoClearTimeout: 30,
    doubleClickEnabled: false,
    autoPasteEnabled: false,
    autoPasteMode: 'always'
  };

  // ── Descriptions ──
  var MODE_DESCRIPTIONS = {
    unicode: 'Sends characters as Unicode events. Works for all characters including special symbols. Best for local VMs and simple remote sessions.',
    vkey: 'Simulates real key presses like a physical keyboard. Required for nested remote sessions (e.g. Horizon VDI → vCenter Console → VM). Only supports characters available on your keyboard layout.'
  };

  var INPUT_MODE_DESCRIPTIONS = {
    single: 'Standard single-line input with optional password masking. Enter triggers paste.',
    multi: 'Multi-line text area for scripts and multi-line content. Ctrl+Enter triggers paste, Enter adds a new line. Password masking is not available in this mode.'
  };

  function updateKeyboardModeDetails() {
    keyboardModeDetails.textContent = MODE_DESCRIPTIONS[keyboardModeSelect.value] || '';
    targetLayoutGroup.style.display = keyboardModeSelect.value === 'vkey' ? '' : 'none';
  }

  function updateInputModeDetails() {
    inputModeDetails.textContent = INPUT_MODE_DESCRIPTIONS[inputModeSelect.value] || '';
  }

  // ── Load stored values ──
  async function loadSettings() {
    try {
      var hotkey = await store.get('activateHotkey');
      activateHotkeyInput.value = hotkey != null ? hotkey : DEFAULTS.activateHotkey;

      var td = await store.get('typingDelay');
      typingDelaySlider.value = td != null ? td : DEFAULTS.typingDelay;
      typingDelayValue.textContent = typingDelaySlider.value + ' ms';

      var kpd = await store.get('keyPressDelay');
      keyPressDelaySlider.value = kpd != null ? kpd : DEFAULTS.keyPressDelay;
      keyPressDelayValue.textContent = keyPressDelaySlider.value + ' ms';

      var km = await store.get('keyboardMode');
      keyboardModeSelect.value = km != null ? km : DEFAULTS.keyboardMode;

      var tl = await store.get('targetLayout');
      targetLayoutSelect.value = tl != null ? tl : DEFAULTS.targetLayout;

      var cai = await store.get('clearAutoIndent');
      clearAutoIndentCb.checked = cai != null ? cai : DEFAULTS.clearAutoIndent;

      updateKeyboardModeDetails();

      var im = await store.get('inputMode');
      inputModeSelect.value = im != null ? im : DEFAULTS.inputMode;
      updateInputModeDetails();

      var aot = await store.get('alwaysOnTop');
      alwaysOnTopCb.checked = aot != null ? aot : DEFAULTS.alwaysOnTop;

      var start = await store.get('autostart');
      autostartCb.checked = start != null ? start : DEFAULTS.autostart;

      var sm = await store.get('startMinimized');
      startMinimizedCb.checked = sm != null ? sm : DEFAULTS.startMinimized;

      var acu = await store.get('autoCheckUpdates');
      autoCheckUpdatesCb.checked = acu != null ? acu : DEFAULTS.autoCheckUpdates;

      var ace = await store.get('autoClearEnabled');
      autoClearEnabledCb.checked = ace != null ? ace : DEFAULTS.autoClearEnabled;

      var act = await store.get('autoClearTimeout');
      autoClearTimeoutSlider.value = act != null ? act : DEFAULTS.autoClearTimeout;
      autoClearTimeoutValue.textContent = autoClearTimeoutSlider.value + ' s';

      updateAutoClearVisibility();

      var dce = await store.get('doubleClickEnabled');
      doubleClickEnabledCb.checked = dce != null ? dce : DEFAULTS.doubleClickEnabled;

      var ape = await store.get('autoPasteEnabled');
      autoPasteEnabledCb.checked = ape != null ? ape : DEFAULTS.autoPasteEnabled;

      var apm = await store.get('autoPasteMode');
      autoPasteModeSelect.value = apm != null ? apm : DEFAULTS.autoPasteMode;

      updateAutoPasteVisibility();
    } catch (err) {
      console.error('Failed to load settings values:', err);
    }
  }

  // ── Slider live feedback ──
  typingDelaySlider.addEventListener('input', function () {
    typingDelayValue.textContent = typingDelaySlider.value + ' ms';
  });

  keyPressDelaySlider.addEventListener('input', function () {
    keyPressDelayValue.textContent = keyPressDelaySlider.value + ' ms';
  });

  autoClearTimeoutSlider.addEventListener('input', function () {
    autoClearTimeoutValue.textContent = autoClearTimeoutSlider.value + ' s';
  });

  function updateAutoClearVisibility() {
    autoClearTimeoutGroup.style.display = autoClearEnabledCb.checked ? '' : 'none';
  }

  autoClearEnabledCb.addEventListener('change', updateAutoClearVisibility);

  function updateAutoPasteVisibility() {
    autoPasteModeGroup.style.display = autoPasteEnabledCb.checked ? '' : 'none';
  }

  autoPasteEnabledCb.addEventListener('change', updateAutoPasteVisibility);

  // ── Description updates ──
  keyboardModeSelect.addEventListener('change', function () {
    updateKeyboardModeDetails();
    fitWindowHeight();
  });
  inputModeSelect.addEventListener('change', updateInputModeDetails);

  // ── Reset buttons ──
  document.querySelectorAll('.btn-reset').forEach(function (btn) {
    btn.addEventListener('click', function () {
      var targetId = btn.getAttribute('data-target');
      var defaultVal = btn.getAttribute('data-default');
      var unit = btn.getAttribute('data-unit') || 'ms';
      var slider = document.getElementById(targetId);
      var valueSpan = document.getElementById(targetId + 'Value');
      if (slider && defaultVal) {
        slider.value = defaultVal;
        if (valueSpan) valueSpan.textContent = defaultVal + ' ' + unit;
      }
    });
  });

  // ── Hotkey recorder ──
  var isRecording = false;

  activateHotkeyInput.addEventListener('click', function () {
    isRecording = true;
    activateHotkeyInput.value = 'Press shortcut…';
    activateHotkeyInput.classList.add('recording');
  });

  activateHotkeyInput.addEventListener('keydown', function (e) {
    if (!isRecording) return;
    e.preventDefault();
    e.stopPropagation();

    if (['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) return;

    var parts = [];
    if (e.ctrlKey) parts.push('Ctrl');
    if (e.altKey) parts.push('Alt');
    if (e.shiftKey) parts.push('Shift');
    if (e.metaKey) parts.push('Super');

    var key = e.key;
    if (key === ' ') key = 'Space';
    else if (key.length === 1) key = key.toUpperCase();
    else if (key === 'ArrowUp') key = 'Up';
    else if (key === 'ArrowDown') key = 'Down';
    else if (key === 'ArrowLeft') key = 'Left';
    else if (key === 'ArrowRight') key = 'Right';

    parts.push(key);
    activateHotkeyInput.value = parts.join('+');
    activateHotkeyInput.classList.remove('recording');
    isRecording = false;
  });

  activateHotkeyInput.addEventListener('blur', function () {
    if (isRecording) {
      store.get('activateHotkey').then(function (v) {
        activateHotkeyInput.value = v || DEFAULTS.activateHotkey;
      });
      activateHotkeyInput.classList.remove('recording');
      isRecording = false;
    }
  });

  clearActivateBtn.addEventListener('click', function () {
    activateHotkeyInput.value = '';
    isRecording = false;
    activateHotkeyInput.classList.remove('recording');
  });

  // ── Save ──
  saveBtn.addEventListener('click', async function () {
    saveBtn.disabled = true;
    saveBtn.textContent = 'Saving…';

    try {
      await store.set('typingDelay', parseInt(typingDelaySlider.value, 10));
      await store.set('keyPressDelay', parseInt(keyPressDelaySlider.value, 10));
      await store.set('keyboardMode', keyboardModeSelect.value);
      await store.set('targetLayout', targetLayoutSelect.value);
      await store.set('clearAutoIndent', clearAutoIndentCb.checked);
      await store.set('inputMode', inputModeSelect.value);
      await store.set('alwaysOnTop', alwaysOnTopCb.checked);
      await store.set('autostart', autostartCb.checked);
      await store.set('startMinimized', startMinimizedCb.checked);
      await store.set('autoCheckUpdates', autoCheckUpdatesCb.checked);
      await store.set('autoClearEnabled', autoClearEnabledCb.checked);
      await store.set('autoClearTimeout', parseInt(autoClearTimeoutSlider.value, 10));
      await store.set('doubleClickEnabled', doubleClickEnabledCb.checked);
      await store.set('autoPasteEnabled', autoPasteEnabledCb.checked);
      await store.set('autoPasteMode', autoPasteModeSelect.value);

      // Apply always-on-top to main window
      try {
        var mainWindow = window.__TAURI__.window.WebviewWindow.getByLabel('main');
        if (mainWindow) {
          await mainWindow.setAlwaysOnTop(alwaysOnTopCb.checked);
        }
      } catch (e) { console.warn('Could not set always-on-top:', e); }

      // Apply autostart
      try {
        if (autostartCb.checked) {
          await window.__TAURI__.autostart.enable();
        } else {
          await window.__TAURI__.autostart.disable();
        }
      } catch (e) { console.warn('Could not toggle autostart:', e); }

      // Update activate hotkey in backend
      try {
        await window.__TAURI__.core.invoke('update_hotkey', {
          newHotkey: activateHotkeyInput.value || ''
        });
      } catch (e) { console.warn('Could not update hotkey:', e); }

      await store.set('activateHotkey', activateHotkeyInput.value || '');
      await store.save();

      // Notify main window
      try {
        await window.__TAURI__.event.emit('settings-changed', {
          activateHotkey: activateHotkeyInput.value,
          typingDelay: parseInt(typingDelaySlider.value, 10),
          keyPressDelay: parseInt(keyPressDelaySlider.value, 10),
          keyboardMode: keyboardModeSelect.value,
          targetLayout: targetLayoutSelect.value,
          clearAutoIndent: clearAutoIndentCb.checked,
          inputMode: inputModeSelect.value,
          alwaysOnTop: alwaysOnTopCb.checked,
          autoCheckUpdates: autoCheckUpdatesCb.checked,
          autoClearEnabled: autoClearEnabledCb.checked,
          autoClearTimeout: parseInt(autoClearTimeoutSlider.value, 10),
          doubleClickEnabled: doubleClickEnabledCb.checked,
          autoPasteEnabled: autoPasteEnabledCb.checked,
          autoPasteMode: autoPasteModeSelect.value
        });
      } catch (e) { console.warn('Could not emit settings-changed:', e); }

      saveBtn.textContent = 'Saved ✓';
      saveBtn.style.background = '#5cb85c';
      setTimeout(async function () {
        try {
          var currentWindow = window.__TAURI__.window.getCurrentWindow();
          await currentWindow.close();
        } catch (e) {
          saveBtn.textContent = 'Save';
          saveBtn.style.background = '';
          saveBtn.disabled = false;
        }
      }, 500);

    } catch (err) {
      console.error('Failed to save settings:', err);
      saveBtn.textContent = 'Error — retry';
      saveBtn.style.background = '#d9534f';
      saveBtn.disabled = false;
      setTimeout(function () {
        saveBtn.textContent = 'Save';
        saveBtn.style.background = '';
      }, 3000);
    }
  });

  // ── Check for Updates (consent-based) ──
  checkUpdatesBtn.addEventListener('click', async function () {
    checkUpdatesBtn.disabled = true;
    checkUpdatesBtn.textContent = 'Checking…';
    updateStatusEl.textContent = '';
    updateStatusEl.className = 'update-status';
    updateBanner.style.display = 'none';

    try {
      var updater = window.__TAURI__.updater;
      if (!updater || !updater.check) {
        throw new Error('Updater API not available');
      }

      var checkOpts = {};
      try {
        var sysProxy = await window.__TAURI__.core.invoke('get_system_proxy');
        if (sysProxy) checkOpts.proxy = sysProxy;
      } catch (e) { /* no proxy */ }

      var update = await updater.check(checkOpts);

      if (update && update.version && isNewerVersion(update.version, currentAppVersion)) {
        pendingUpdate = update;
        updateStatusEl.textContent = '';
        updateStatusEl.className = 'update-status';
        checkUpdatesBtn.textContent = 'Check for Updates';
        checkUpdatesBtn.disabled = false;
        showUpdateBanner(update.version);
      } else {
        updateStatusEl.textContent = 'Up to date (v' + (currentAppVersion || '?') + ')';
        updateStatusEl.className = 'update-status update-uptodate';
        checkUpdatesBtn.textContent = 'Check for Updates';
        checkUpdatesBtn.disabled = false;
      }
    } catch (err) {
      console.error('Update check failed:', err);
      var errMsg = String(err.message || err);
      if (errMsg.indexOf('network') >= 0 || errMsg.indexOf('connect') >= 0 || errMsg.indexOf('proxy') >= 0 || errMsg.indexOf('tls') >= 0 || errMsg.indexOf('ssl') >= 0) {
        updateStatusEl.textContent = 'Network error — check proxy/firewall (set HTTPS_PROXY env var if behind a proxy)';
      } else {
        updateStatusEl.textContent = 'Check failed: ' + errMsg;
      }
      updateStatusEl.className = 'update-status update-error';
      checkUpdatesBtn.textContent = 'Check for Updates';
      checkUpdatesBtn.disabled = false;
    }
  });

  function showUpdateBanner(version) {
    updateBannerVersion.textContent = 'v' + version;
    updateBanner.style.display = 'block';
    updateProgress.style.display = 'none';
    updateInstallBtn.disabled = false;
    updateInstallBtn.textContent = 'Install & Restart';
  }

  updateDismissBtn.addEventListener('click', function () {
    updateBanner.style.display = 'none';
    pendingUpdate = null;
    window.__TAURI__.event.emit('update-dismissed', {});
  });

  updateInstallBtn.addEventListener('click', async function () {
    if (!pendingUpdate) return;
    updateInstallBtn.disabled = true;
    updateInstallBtn.textContent = 'Installing…';
    updateProgress.style.display = '';

    var progressFill = updateProgress.querySelector('.update-progress-fill');
    var progressText = updateProgress.querySelector('.update-progress-text');

    try {
      await pendingUpdate.downloadAndInstall(function (event) {
        if (event.event === 'Started' && event.data && event.data.contentLength) {
          progressText.textContent = 'Downloading…';
        } else if (event.event === 'Progress' && event.data) {
          var total = event.data.contentLength || 0;
          var chunk = event.data.chunkLength || 0;
          if (total > 0) {
            var pct = Math.min(100, Math.round((chunk / total) * 100));
            progressFill.style.width = pct + '%';
          }
        } else if (event.event === 'Finished') {
          progressFill.style.width = '100%';
          progressText.textContent = 'Installing…';
        }
      });
      progressText.textContent = 'Restarting…';
      try { await window.__TAURI__.core.invoke('plugin:updater|restart'); } catch (e) {
        await window.__TAURI__.process.relaunch();
      }
    } catch (err) {
      console.error('Update install failed:', err);
      updateInstallBtn.textContent = 'Install & Restart';
      updateInstallBtn.disabled = false;
      progressText.textContent = 'Installation failed';
    }
  });

  // ── Listen for update-available event from backend ──
  window.__TAURI__.event.listen('update-available', function (event) {
    if (event.payload && event.payload.version && isNewerVersion(event.payload.version, currentAppVersion)) {
      // Navigate to Updates section
      navItems.forEach(function (n) { n.classList.remove('active'); });
      sections.forEach(function (s) { s.classList.remove('active'); });
      document.querySelector('[data-section="updates"]').classList.add('active');
      document.getElementById('section-updates').classList.add('active');

      // We need the actual update object to install, so trigger a check
      checkUpdatesBtn.click();
    }
  });

  // ── Listen for mode changes from main window ──
  window.__TAURI__.event.listen('mode-changed', function (event) {
    var km = event.payload && event.payload.keyboardMode;
    if (km != null) {
      keyboardModeSelect.value = km;
      updateKeyboardModeDetails();
    }
  });

  // Every disclosure changes the content height — re-fit when one opens or closes.
  document.querySelectorAll('details').forEach(function (d) {
    d.addEventListener('toggle', fitWindowHeight);
  });

  await loadSettings();
  // loadSettings() decides whether Target Layout is visible, so the first fit has to
  // run after it.
  fitWindowHeight();
})();
