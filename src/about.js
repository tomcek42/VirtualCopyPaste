/**
 * About page controller.
 * Loads version from backend, handles GitHub link and close.
 */
(async function () {
  // Wait for Tauri API
  async function waitForTauri(maxMs) {
    var start = Date.now();
    while (!window.__TAURI__ || !window.__TAURI__.core) {
      if (Date.now() - start > maxMs) throw new Error('Tauri API not available');
      await new Promise(function (r) { setTimeout(r, 50); });
    }
  }

  try {
    await waitForTauri(3000);
  } catch (e) {
    return;
  }

  // Load version
  try {
    var version = await window.__TAURI__.core.invoke('get_version');
    document.getElementById('version').textContent = 'Version ' + version;
  } catch (e) {
    document.getElementById('version').textContent = '';
  }

  // GitHub button
  document.getElementById('githubBtn').addEventListener('click', function () {
    window.__TAURI__.core.invoke('open_url', { url: 'https://github.com/tomcek42/VirtualCopyPaste' })
      .catch(function (err) { console.warn('Could not open URL:', err); });
  });

  // Close button
  document.getElementById('closeBtn').addEventListener('click', function () {
    window.__TAURI__.window.getCurrentWindow().close();
  });

  // ESC to close
  document.addEventListener('keydown', function (e) {
    if (e.key === 'Escape') {
      window.__TAURI__.window.getCurrentWindow().close();
    }
  });
})();
