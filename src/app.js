/**
 * Virtual Copy Paste — App Controller
 * Loads settings from tauri-plugin-store. Opens settings window.
 * Paste switches to next window (like ALT+TAB) then types.
 */
(async function () {
  var textInput = document.querySelector('#textInput');
  var pasteBtn = document.querySelector('#pasteBtn');
  var statusEl = document.querySelector('#status');
  var maskToggle = document.querySelector('#maskToggle');
  var inputRow = textInput.parentElement;
  var clearBtn = document.querySelector('#clearInput');
  var modeToggle = document.querySelector('#modeToggle');
  var isPasting = false;
  var isMasked = false;
  var inputMode = 'single';

  // ── Clear button visibility ──
  function updateClearButton() {
    clearBtn.style.display = textInput.value.length > 0 ? '' : 'none';
  }

  function handleClear() {
    textInput.value = '';
    updateClearButton();
    textInput.focus();
    // Trigger yeti eye reset
    if (window.yetiAnimation) window.yetiAnimation.resetFace();
  }

  // ── Settings (loaded from store, with defaults) ──
  var typingDelay = 20;
  var keyPressDelay = 5;
  var keyboardMode = 'unicode';

  // ── Window sizing per input mode ──
  var WINDOW_WIDTH = 300;
  var SINGLE_LINE_HEIGHT = 270;
  var MULTI_LINE_HEIGHT = 300;
  var UPDATE_NOTICE_HEIGHT = 28;
  var updateNoticeVisible = false;

  function resizeWindowForMode(mode) {
    try {
      var win = window.__TAURI__.window.getCurrentWindow();
      var h = mode === 'multi' ? MULTI_LINE_HEIGHT : SINGLE_LINE_HEIGHT;
      if (updateNoticeVisible) h += UPDATE_NOTICE_HEIGHT;
      win.setSize(new window.__TAURI__.window.LogicalSize(WINDOW_WIDTH, h));
    } catch (e) { console.warn('Could not resize window:', e); }
  }

  // ── Input mode switching ──
  function applyInputMode(mode) {
    var oldValue = textInput.value;
    inputMode = mode;

    if (mode === 'multi') {
      // Switch to textarea if currently an input
      if (textInput.tagName === 'INPUT') {
        var ta = document.createElement('textarea');
        ta.id = 'textInput';
        ta.autocomplete = 'off';
        ta.rows = 3;
        ta.placeholder = 'Paste multi-line text here (Ctrl+Enter to send)';
        ta.value = oldValue;
        inputRow.replaceChild(ta, textInput);
        textInput = ta;
        attachInputListeners();
        if (window.yetiAnimation) window.yetiAnimation.rebind();
      }
      // Hide mask toggle — masking doesn't apply to multi-line
      maskToggle.style.display = 'none';
      // Move clear button to mask toggle's position
      clearBtn.style.right = '2px';
      clearBtn.style.top = '8px';
      clearBtn.style.transform = 'none';
      // Reset mask state
      if (isMasked) {
        isMasked = false;
        if (window.yetiAnimation) window.yetiAnimation.uncoverEyes();
      }
    } else {
      // Switch to input if currently a textarea
      if (textInput.tagName === 'TEXTAREA') {
        var inp = document.createElement('input');
        inp.type = 'text';
        inp.id = 'textInput';
        inp.autocomplete = 'off';
        inp.placeholder = 'Please enter text to copy paste into the VM!';
        inp.value = oldValue.replace(/[\r\n]+/g, ' ');
        inputRow.replaceChild(inp, textInput);
        textInput = inp;
        attachInputListeners();
        if (window.yetiAnimation) window.yetiAnimation.rebind();
      }
      // Show mask toggle
      maskToggle.style.display = '';
      // Reset clear button position
      clearBtn.style.right = '30px';
      clearBtn.style.top = '50%';
      clearBtn.style.transform = 'translateY(-50%)';
    }

    resizeWindowForMode(mode);
  }

  function attachInputListeners() {
    textInput.addEventListener('keydown', handleKeyDown);
    textInput.addEventListener('input', updateClearButton);
    updateClearButton();
  }

  function handleKeyDown(e) {
    if (isPasting) return;
    if (inputMode === 'multi') {
      // Ctrl+Enter = paste, Enter = newline
      if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        handlePaste();
      }
    } else {
      // Enter = paste
      if (e.key === 'Enter') {
        e.preventDefault();
        handlePaste();
      }
    }
  }

  async function loadSettings() {
    try {
      var store = await window.__TAURI__.store.load('settings.json', { autoSave: false });
      var td = await store.get('typingDelay');
      if (td != null) typingDelay = td;
      var kpd = await store.get('keyPressDelay');
      if (kpd != null) keyPressDelay = kpd;
      var km = await store.get('keyboardMode');
      if (km != null) keyboardMode = km;

      var im = await store.get('inputMode');
      if (im != null) applyInputMode(im);

      // Apply always-on-top
      var aot = await store.get('alwaysOnTop');
      if (aot) {
        var win = window.__TAURI__.window.getCurrentWindow();
        await win.setAlwaysOnTop(true);
      }
    } catch (e) {
      console.warn('Failed to load settings:', e);
    }
  }

  // ── Listen for settings changes ──
  window.__TAURI__.event.listen('settings-changed', function (event) {
    var s = event.payload;
    if (s.typingDelay != null) typingDelay = s.typingDelay;
    if (s.keyPressDelay != null) keyPressDelay = s.keyPressDelay;
    if (s.keyboardMode != null) { keyboardMode = s.keyboardMode; updateModeToggle(); }
    if (s.inputMode != null) applyInputMode(s.inputMode);
  });

  // ── Listen for paste status updates from backend ──
  window.__TAURI__.event.listen('paste-status', function (event) {
    var status = event.payload;
    if (status === 'waiting-for-click') {
      setStatus('Click in target window...', 'waiting');
      pasteBtn.textContent = 'Click target...';
    } else if (status === 'typing') {
      setStatus('', '');
      pasteBtn.textContent = 'Typing...';
    }
  });

  // ── Paste ──
  async function handlePaste() {
    var text = textInput.value;
    if (!text || isPasting) return;

    isPasting = true;
    pasteBtn.disabled = true;
    pasteBtn.classList.add('pasting');
    pasteBtn.textContent = 'Switching...';

    try {
      var result = await window.__TAURI__.core.invoke('type_text', {
        text: text,
        delayMs: typingDelay,
        keyboardMode: keyboardMode,
        keyPressDelay: keyPressDelay
      });
      setStatus(result, 'success');
    } catch (err) {
      setStatus(err, 'error');
    }

    pasteBtn.textContent = 'Paste to Target';
    pasteBtn.disabled = false;
    pasteBtn.classList.remove('pasting');
    isPasting = false;

    setTimeout(function () { if (!isPasting) setStatus('', ''); }, 4000);
  }

  // ── Mask toggle (single-line only) ──
  function handleMaskToggle() {
    if (inputMode === 'multi') return;
    isMasked = !isMasked;
    var icon = maskToggle.querySelector('.mask-icon');
    if (isMasked) {
      textInput.type = 'password';
      if (icon) icon.src = 'eye-off.svg';
      if (window.yetiAnimation) window.yetiAnimation.coverEyes();
    } else {
      textInput.type = 'text';
      if (icon) icon.src = 'eye.svg';
      if (window.yetiAnimation) window.yetiAnimation.uncoverEyes();
    }
  }

  function setStatus(text, type) {
    statusEl.textContent = text;
    statusEl.className = 'status ' + (type || '');
  }

  // ── Mode toggle ──
  function updateModeToggle() {
    if (keyboardMode === 'vkey') {
      modeToggle.classList.add('compat');
      modeToggle.title = 'Keyboard mode: Compatible (VDI/Remote) — click to switch to Standard';
    } else {
      modeToggle.classList.remove('compat');
      modeToggle.title = 'Keyboard mode: Standard (Unicode) — click to switch to Compatible';
    }
  }

  async function handleModeToggleClick() {
    keyboardMode = keyboardMode === 'unicode' ? 'vkey' : 'unicode';
    updateModeToggle();

    var label = keyboardMode === 'vkey' ? 'Compatible' : 'Standard';
    setStatus('Mode: ' + label, 'success');
    setTimeout(function () { if (!isPasting) setStatus('', ''); }, 2000);

    try {
      var store = await window.__TAURI__.store.load('settings.json', { autoSave: false });
      await store.set('keyboardMode', keyboardMode);
      await store.save();
      await window.__TAURI__.event.emit('mode-changed', { keyboardMode: keyboardMode });
    } catch (e) { console.warn('Failed to persist keyboard mode:', e); }
  }

  // ── Event listeners ──
  pasteBtn.addEventListener('click', handlePaste);
  maskToggle.addEventListener('click', handleMaskToggle);
  clearBtn.addEventListener('click', handleClear);
  modeToggle.addEventListener('click', handleModeToggleClick);
  textInput.addEventListener('keydown', handleKeyDown);
  textInput.addEventListener('input', updateClearButton);

  // ── Global ESC handler (works even when input is not focused) ──
  document.addEventListener('keydown', function (e) {
    if (e.key === 'Escape') {
      e.preventDefault();
      try {
        var win = window.__TAURI__.window.getCurrentWindow();
        win.hide();
      } catch (err) { console.warn('Could not hide window:', err); }
    }
  });

  // ── Update notice in main window ──
  var updateNotice = document.getElementById('updateNotice');
  var updateNoticeText = document.getElementById('updateNoticeText');
  var updateNoticeBtn = document.getElementById('updateNoticeBtn');

  var pendingUpdateVersion = null;

  function showUpdateNotice(version) {
    pendingUpdateVersion = version;
    updateNoticeText.textContent = 'Update available: v' + version;
    updateNotice.style.display = '';
    updateNoticeVisible = true;
    resizeWindowForMode(inputMode);
  }

  window.__TAURI__.event.listen('update-available', function (event) {
    if (event.payload && event.payload.version) {
      showUpdateNotice(event.payload.version);
    }
  });

  // Check for updates from JS side to avoid race with backend event
  (async function () {
    try {
      var store = await window.__TAURI__.store.load('settings.json', { autoSave: false });
      var autoCheck = await store.get('autoCheckUpdates');
      if (autoCheck === false) return;

      var updater = window.__TAURI__.updater;
      if (!updater || !updater.check) return;

      var update = await updater.check();
      if (update && update.version) {
        showUpdateNotice(update.version);
      }
    } catch (e) {
      console.warn('Frontend update check failed:', e);
    }
  })();

  if (updateNoticeBtn) {
    updateNoticeBtn.addEventListener('click', function () {
      openSettingsWindow();
      if (pendingUpdateVersion) {
        setTimeout(function () {
          window.__TAURI__.event.emit('update-available', { version: pendingUpdateVersion });
        }, 1500);
      }
    });
  }

  function openSettingsWindow() {
    window.__TAURI__.core.invoke('open_settings').catch(function (e) {
      console.error('Could not open settings:', e);
    });
  }

  // ── Init ──
  await loadSettings();
  updateModeToggle();
})();
