/**
 * Settings page controller.
 * Uses tauri-plugin-store for persistence and tauri-plugin-autostart for startup.
 */
(async function () {
  // Wait for Tauri to inject __TAURI__ into this webview
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
    store = await window.__TAURI__.store.load('settings.json', { autoSave: false });
  } catch (err) {
    console.error('Failed to load store:', err);
    document.body.innerHTML = '<p style="color:#d9534f;padding:20px;">Failed to load settings store. Check console.</p>';
    return;
  }

  // ── DOM refs ──
  var activateHotkeyInput = document.getElementById('activateHotkey');
  var clearActivateBtn = document.getElementById('clearActivateHotkey');
  var typingDelaySlider = document.getElementById('typingDelay');
  var typingDelayValue = document.getElementById('typingDelayValue');
  var keyboardModeSelect = document.getElementById('keyboardMode');
  var keyboardModeDetails = document.getElementById('keyboardModeDetails');
  var inputModeSelect = document.getElementById('inputMode');
  var inputModeDetails = document.getElementById('inputModeDetails');
  var alwaysOnTopCb = document.getElementById('alwaysOnTop');
  var autostartCb = document.getElementById('autostart');
  var startMinimizedCb = document.getElementById('startMinimized');
  var saveBtn = document.getElementById('saveSettings');

  // ── Defaults ──
  var DEFAULTS = {
    activateHotkey: 'Ctrl+Shift+Space',
    typingDelay: 20,
    keyboardMode: 'unicode',
    inputMode: 'single',
    alwaysOnTop: false,
    autostart: false,
    startMinimized: false
  };

  // ── Keyboard mode descriptions ──
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

      var km = await store.get('keyboardMode');
      keyboardModeSelect.value = km != null ? km : DEFAULTS.keyboardMode;
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
    } catch (err) {
      console.error('Failed to load settings values:', err);
    }
  }

  // ── Slider live feedback ──
  typingDelaySlider.addEventListener('input', function () {
    typingDelayValue.textContent = typingDelaySlider.value + ' ms';
  });

  // ── Keyboard mode description update ──
  keyboardModeSelect.addEventListener('change', updateKeyboardModeDetails);
  inputModeSelect.addEventListener('change', updateInputModeDetails);

  // ── Reset buttons ──
  document.querySelectorAll('.btn-reset').forEach(function (btn) {
    btn.addEventListener('click', function () {
      var targetId = btn.getAttribute('data-target');
      var defaultVal = btn.getAttribute('data-default');
      var slider = document.getElementById(targetId);
      var valueSpan = document.getElementById(targetId + 'Value');
      if (slider && defaultVal) {
        slider.value = defaultVal;
        if (valueSpan) valueSpan.textContent = defaultVal + ' ms';
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
      await store.set('keyboardMode', keyboardModeSelect.value);
      await store.set('inputMode', inputModeSelect.value);
      await store.set('alwaysOnTop', alwaysOnTopCb.checked);
      await store.set('autostart', autostartCb.checked);
      await store.set('startMinimized', startMinimizedCb.checked);

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

      // Update activate hotkey in backend (unregister old, register new)
      try {
        var oldHotkey = await store.get('activateHotkey');
        await window.__TAURI__.core.invoke('update_hotkey', {
          oldHotkey: oldHotkey || null,
          newHotkey: activateHotkeyInput.value || ''
        });
      } catch (e) { console.warn('Could not update hotkey:', e); }

      // Save the new hotkey value to store (after successful registration)
      await store.set('activateHotkey', activateHotkeyInput.value || '');
      await store.save();

      // Notify main window to update delays, hotkey, and keyboard mode
      try {
        await window.__TAURI__.event.emit('settings-changed', {
          activateHotkey: activateHotkeyInput.value,
          typingDelay: parseInt(typingDelaySlider.value, 10),
          keyboardMode: keyboardModeSelect.value,
          inputMode: inputModeSelect.value,
          alwaysOnTop: alwaysOnTopCb.checked
        });
      } catch (e) { console.warn('Could not emit settings-changed:', e); }

      // Brief "Saved" feedback, then close
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

  window.__TAURI__.event.listen('mode-changed', function (event) {
    var km = event.payload && event.payload.keyboardMode;
    if (km != null) {
      keyboardModeSelect.value = km;
      updateKeyboardModeDetails();
    }
  });

  await loadSettings();
})();
