/**
 * Virtual Copy Paste — App Controller
 * No slots. Paste switches to next window (like ALT+TAB) then types.
 */
(function () {
  var textInput = document.querySelector('#textInput');
  var pasteBtn = document.querySelector('#pasteBtn');
  var statusEl = document.querySelector('#status');
  var maskToggle = document.querySelector('#maskToggle');
  var isPasting = false;
  var isMasked = false;

  async function handlePaste() {
    var text = textInput.value;
    if (!text || isPasting) return;

    isPasting = true;
    pasteBtn.disabled = true;
    pasteBtn.classList.add('pasting');
    setStatus('Typing...', 'typing');
    pasteBtn.textContent = 'Typing...';

    try {
      var result = await window.__TAURI__.core.invoke('type_text', {
        text: text,
        delayMs: 20
      });
      setStatus(result, 'success');
    } catch (err) {
      setStatus(err, 'error');
    }

    pasteBtn.textContent = 'Paste to Target';
    pasteBtn.disabled = false;
    pasteBtn.classList.remove('pasting');
    isPasting = false;

    setTimeout(function () { if (!isPasting) setStatus('', ''); }, 3000);
  }

  function handleMaskToggle() {
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

  pasteBtn.addEventListener('click', handlePaste);
  maskToggle.addEventListener('click', handleMaskToggle);
  textInput.addEventListener('keydown', function (e) {
    if (e.key === 'Enter' && !isPasting) handlePaste();
  });
})();
