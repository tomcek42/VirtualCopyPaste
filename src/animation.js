/**
 * Yeti Animation — adapted from Darin Senneff's original animated SVG avatar.
 * Uses GSAP v3 for eye-tracking and arm-cover animations.
 * Mouth stays closed at all times (no morph transitions).
 * Selectors adapted from justin-calleja/login-page cartoon.js.
 */
(function () {
  var email = document.querySelector('#textInput'),
      mySVG = document.querySelector('.cartoon-container'),
      armL = document.querySelector('.armL'),
      armR = document.querySelector('.armR'),
      eyeL = document.querySelector('.eyeL'),
      eyeR = document.querySelector('.eyeR'),
      nose = document.querySelector('.nose'),
      mouth = document.querySelector('.mouth'),
      chin = document.querySelector('.chin'),
      face = document.querySelector('.face'),
      eyebrow = document.querySelector('.eyebrow'),
      outerEarL = document.querySelector('.earL .outerEar'),
      outerEarR = document.querySelector('.earR .outerEar'),
      earHairL = document.querySelector('.earL .earHair'),
      earHairR = document.querySelector('.earR .earHair'),
      hair = document.querySelector('.hair');

  var eyeMaxHorizD = 20, eyeMaxVertD = 10, noseMaxHorizD = 23, noseMaxVertD = 10;
  var screenCenter, svgCoords;

  function getAngle(x1, y1, x2, y2) { return Math.atan2(y1 - y2, x1 - x2); }

  function getPosition(el) {
    var xPos = 0, yPos = 0;
    while (el) {
      if (el.tagName == "BODY") {
        xPos += (el.offsetLeft - (el.scrollLeft || document.documentElement.scrollLeft) + el.clientLeft);
        yPos += (el.offsetTop - (el.scrollTop || document.documentElement.scrollTop) + el.clientTop);
      } else {
        xPos += (el.offsetLeft - el.scrollLeft + el.clientLeft);
        yPos += (el.offsetTop - el.scrollTop + el.clientTop);
      }
      el = el.offsetParent;
    }
    return { x: xPos, y: yPos };
  }

  function getCoord() {
    var carPos = email.selectionEnd,
        div = document.createElement('div'),
        span = document.createElement('span'),
        copyStyle = getComputedStyle(email);
    [].forEach.call(copyStyle, function (prop) { div.style[prop] = copyStyle[prop]; });
    div.style.position = 'absolute';
    div.style.visibility = 'hidden';
    document.body.appendChild(div);
    div.textContent = email.value.substr(0, carPos);
    span.textContent = email.value.substr(carPos) || '.';
    div.appendChild(span);

    var emailCoords = getPosition(email);
    var caretCoords = getPosition(span);
    svgCoords = getPosition(mySVG);
    screenCenter = svgCoords.x + (mySVG.offsetWidth / 2);
    var caretPos = caretCoords.x + emailCoords.x;
    var dFromC = screenCenter - caretPos;

    var eyeLCoords = { x: svgCoords.x + 84, y: svgCoords.y + 76 };
    var eyeRCoords = { x: svgCoords.x + 113, y: svgCoords.y + 76 };
    var noseCoords = { x: svgCoords.x + 97, y: svgCoords.y + 81 };
    var mouthCoords = { x: svgCoords.x + 100, y: svgCoords.y + 100 };
    var targetX = emailCoords.x + caretCoords.x, targetY = emailCoords.y + 25;

    var eyeLAngle = getAngle(eyeLCoords.x, eyeLCoords.y, targetX, targetY);
    var eyeRAngle = getAngle(eyeRCoords.x, eyeRCoords.y, targetX, targetY);
    var noseAngle = getAngle(noseCoords.x, noseCoords.y, targetX, targetY);
    var mouthAngle = getAngle(mouthCoords.x, mouthCoords.y, targetX, targetY);

    var mouthX = Math.cos(mouthAngle) * noseMaxHorizD;
    var mouthY = Math.sin(mouthAngle) * noseMaxVertD;
    var mouthR = Math.cos(mouthAngle) * 6;
    var chinS = 1 - ((dFromC * .15) / 100);
    if (chinS > 1) chinS = 1 - (chinS - 1);

    gsap.to(eyeL, { duration: 1, x: -(Math.cos(eyeLAngle) * eyeMaxHorizD), y: -(Math.sin(eyeLAngle) * eyeMaxVertD), ease: 'expo.out' });
    gsap.to(eyeR, { duration: 1, x: -(Math.cos(eyeRAngle) * eyeMaxHorizD), y: -(Math.sin(eyeRAngle) * eyeMaxVertD), ease: 'expo.out' });
    gsap.to(nose, { duration: 1, x: -(Math.cos(noseAngle) * noseMaxHorizD), y: -(Math.sin(noseAngle) * noseMaxVertD), rotation: mouthR, transformOrigin: 'center center', ease: 'expo.out' });
    gsap.to(mouth, { duration: 1, x: -mouthX, y: -mouthY, rotation: mouthR, transformOrigin: 'center center', ease: 'expo.out' });
    gsap.to(chin, { duration: 1, x: -mouthX * .8, y: -mouthY * .5, scaleY: chinS, ease: 'expo.out' });
    gsap.to(face, { duration: 1, x: -mouthX * .3, y: -mouthY * .4, skewX: -(Math.cos(mouthAngle) * 5), transformOrigin: 'center top', ease: 'expo.out' });
    gsap.to(eyebrow, { duration: 1, x: -mouthX * .3, y: -mouthY * .4, skewX: -(Math.cos(mouthAngle) * 25), transformOrigin: 'center top', ease: 'expo.out' });
    gsap.to(outerEarL, { duration: 1, x: Math.cos(mouthAngle) * 4, y: -(Math.cos(mouthAngle) * 5), ease: 'expo.out' });
    gsap.to(outerEarR, { duration: 1, x: Math.cos(mouthAngle) * 4, y: Math.cos(mouthAngle) * 5, ease: 'expo.out' });
    gsap.to(earHairL, { duration: 1, x: -(Math.cos(mouthAngle) * 4), y: -(Math.cos(mouthAngle) * 5), ease: 'expo.out' });
    gsap.to(earHairR, { duration: 1, x: -(Math.cos(mouthAngle) * 4), y: Math.cos(mouthAngle) * 5, ease: 'expo.out' });
    gsap.to(hair, { duration: 1, x: Math.cos(mouthAngle) * 6, scaleY: 1.2, transformOrigin: 'center bottom', ease: 'expo.out' });

    document.body.removeChild(div);
  }

  function onEmailInput() {
    if (!isCovering) getCoord();
  }

  function onEmailFocus() { if (!isCovering) getCoord(); }
  function onEmailBlur() { if (!isCovering) resetFace(); }

  var isCovering = false;

  function coverEyes() {
    // Kill any in-flight arm/face tweens to prevent race conditions on rapid toggling
    gsap.killTweensOf([armL, armR, eyeL, eyeR, nose, mouth, chin, face, eyebrow,
                       outerEarL, outerEarR, earHairL, earHairR, hair]);
    isCovering = true;

    // Reset face to neutral so eyes don't peek below the hands
    gsap.to([eyeL, eyeR], { duration: .3, x: 0, y: 0, ease: 'expo.out' });
    gsap.to(nose, { duration: .3, x: 0, y: 0, rotation: 0, scaleX: 1, scaleY: 1, ease: 'expo.out' });
    gsap.to(mouth, { duration: .3, x: 0, y: 0, rotation: 0, ease: 'expo.out' });
    gsap.to(chin, { duration: .3, x: 0, y: 0, scaleY: 1, ease: 'expo.out' });
    gsap.to([face, eyebrow], { duration: .3, x: 0, y: 0, skewX: 0, ease: 'expo.out' });
    gsap.to([outerEarL, outerEarR, earHairL, earHairR, hair], { duration: .3, x: 0, y: 0, scaleY: 1, ease: 'expo.out' });

    // Bring hands up to cover eyes
    gsap.to(armL, { duration: .45, x: -93, y: 2, rotation: 0, ease: 'quad.out' });
    gsap.to(armR, { duration: .45, x: -93, y: 2, rotation: 0, ease: 'quad.out', delay: .1 });
  }

  function uncoverEyes() {
    // Kill any in-flight arm tweens to prevent race conditions on rapid toggling
    gsap.killTweensOf([armL, armR]);
    isCovering = false;

    gsap.to(armL, { duration: 1.35, y: 220, ease: 'quad.out' });
    gsap.to(armL, { duration: 1.35, rotation: 105, ease: 'quad.out', delay: .1 });
    gsap.to(armR, { duration: 1.35, y: 220, ease: 'quad.out' });
    gsap.to(armR, { duration: 1.35, rotation: -105, ease: 'quad.out', delay: .1 });
  }

  function resetFace() {
    gsap.to([eyeL, eyeR], { duration: 1, x: 0, y: 0, ease: 'expo.out' });
    gsap.to(nose, { duration: 1, x: 0, y: 0, scaleX: 1, scaleY: 1, ease: 'expo.out' });
    gsap.to(mouth, { duration: 1, x: 0, y: 0, rotation: 0, ease: 'expo.out' });
    gsap.to(chin, { duration: 1, x: 0, y: 0, scaleY: 1, ease: 'expo.out' });
    gsap.to([face, eyebrow], { duration: 1, x: 0, y: 0, skewX: 0, ease: 'expo.out' });
    gsap.to([outerEarL, outerEarR, earHairL, earHairR, hair], { duration: 1, x: 0, y: 0, scaleY: 1, ease: 'expo.out' });
  }

  // Init
  email.addEventListener('focus', onEmailFocus);
  email.addEventListener('blur', onEmailBlur);
  email.addEventListener('input', onEmailInput);
  gsap.set(armL, { x: -93, y: 220, rotation: 105, transformOrigin: 'top left' });
  gsap.set(armR, { x: -93, y: 220, rotation: -105, transformOrigin: 'top right' });

  function rebind() {
    // Detach from old element
    email.removeEventListener('focus', onEmailFocus);
    email.removeEventListener('blur', onEmailBlur);
    email.removeEventListener('input', onEmailInput);
    // Re-query and attach to current element
    email = document.querySelector('#textInput');
    email.addEventListener('focus', onEmailFocus);
    email.addEventListener('blur', onEmailBlur);
    email.addEventListener('input', onEmailInput);
  }

  // ── Blow animation (smoke cloud, mouth morph, cheek puff) ──
  var mouthBG = document.querySelector('.mouthBG'),
      mouthBlowBG = document.querySelector('.mouthBlowBG'),
      cheekL = document.querySelector('.cheekL'),
      cheekR = document.querySelector('.cheekR'),
      cartoonWrapper = document.getElementById('cartoonWrapper');
  var isBlowing = false;

  function spawnSmokeCloud(onTextClear) {
    var wrapperRect = cartoonWrapper.getBoundingClientRect();
    var inputRect = email.getBoundingClientRect();

    var originX = wrapperRect.left + wrapperRect.width / 2;
    var originY = wrapperRect.top + wrapperRect.height * 0.78;
    var targetX = inputRect.left + inputRect.width * 0.4;
    var targetY = inputRect.top + inputRect.height / 2;

    var puffCount = 5;
    var textCleared = false;
    for (var i = 0; i < puffCount; i++) {
      (function (index) {
        var puff = document.createElement('div');
        puff.className = 'smoke-puff';
        document.body.appendChild(puff);

        var size = 16 + Math.random() * 18;
        var delay = index * 0.05;
        var spreadX = (Math.random() - 0.5) * 24;
        var spreadY = (Math.random() - 0.5) * 16;

        puff.style.width = size + 'px';
        puff.style.height = size + 'px';
        puff.style.left = (originX - size / 2) + 'px';
        puff.style.top = (originY - size / 2) + 'px';

        gsap.to(puff, { duration: 0.12, opacity: 0.7, scale: 1.1, delay: delay, ease: 'power2.out' });
        gsap.to(puff, {
          duration: 0.45 + Math.random() * 0.15,
          x: (targetX - originX) + spreadX,
          y: (targetY - originY) + spreadY,
          scale: 1.5 + Math.random() * 0.4,
          delay: delay + 0.08,
          ease: 'power1.out',
          onComplete: function () {
            if (!textCleared && onTextClear) { textCleared = true; onTextClear(); }
          }
        });
        gsap.to(puff, {
          duration: 0.3, opacity: 0, delay: delay + 0.35, ease: 'power1.in',
          onComplete: function () { if (puff.parentNode) puff.parentNode.removeChild(puff); }
        });
      })(i);
    }
  }

  function resetBlowState() {
    mouthBlowBG.style.display = 'none';
    mouthBG.style.display = 'block';
    gsap.set(mouthBlowBG, { attr: { rx: 4, ry: 3.5 } });
    isBlowing = false;
  }

  function blowText(onDone) {
    if (isBlowing) return;
    isBlowing = true;

    if (isCovering) {
      mouthBG.style.display = 'none';
      mouthBlowBG.style.display = 'block';
      spawnSmokeCloud(function () { if (onDone) onDone(); });
      setTimeout(resetBlowState, 700);
      return;
    }

    gsap.to(cheekL, { duration: 0.3, attr: { rx: 7, ry: 5 }, opacity: 0.55, ease: 'power2.out' });
    gsap.to(cheekR, { duration: 0.3, attr: { rx: 7, ry: 5 }, opacity: 0.55, ease: 'power2.out' });
    gsap.to([eyeL, eyeR], { duration: 0.25, scaleY: 0.7, transformOrigin: 'center center', ease: 'power2.out' });

    setTimeout(function () {
      mouthBG.style.display = 'none';
      mouthBlowBG.style.display = 'block';
      gsap.fromTo(mouthBlowBG, { attr: { rx: 3, ry: 2.5 } }, { duration: 0.2, attr: { rx: 5, ry: 4 }, ease: 'power2.out' });

      gsap.to(cheekL, { duration: 0.4, attr: { rx: 0, ry: 0 }, opacity: 0, ease: 'power2.in' });
      gsap.to(cheekR, { duration: 0.4, attr: { rx: 0, ry: 0 }, opacity: 0, ease: 'power2.in' });
      gsap.to([eyeL, eyeR], { duration: 0.35, scaleY: 1, ease: 'power2.out', delay: 0.2 });

      spawnSmokeCloud(function () { if (onDone) onDone(); });
      setTimeout(resetBlowState, 700);
    }, 300);
  }

  window.yetiAnimation = { coverEyes: coverEyes, uncoverEyes: uncoverEyes, resetFace: resetFace, rebind: rebind, blowText: blowText };
})();

(function () {
  var orbit = document.getElementById('countdownOrbit');
  var orbitProgress = document.getElementById('orbitProgress');
  var secondsBadge = document.getElementById('countdownSeconds');
  var circumference = 2 * Math.PI * 65;

  orbitProgress.style.strokeDasharray = circumference;

  var timer = null;
  var remaining = 0;
  var total = 0;
  var onExpired = null;

  function updateOrbit() {
    var fraction = remaining / total;
    var offset = circumference * (1 - fraction);
    orbitProgress.style.strokeDashoffset = offset;
    secondsBadge.textContent = remaining + 's';

    orbitProgress.classList.remove('warn', 'danger');
    secondsBadge.classList.remove('warn', 'danger');

    if (fraction <= 0.2) {
      orbitProgress.classList.add('danger');
      secondsBadge.classList.add('danger');
    } else if (fraction <= 0.4) {
      orbitProgress.classList.add('warn');
      secondsBadge.classList.add('warn');
    }
  }

  function startCountdown(seconds) {
    clearInterval(timer);
    total = seconds;
    remaining = total;

    orbit.classList.add('active');
    secondsBadge.classList.add('active');

    orbitProgress.style.transition = 'none';
    updateOrbit();

    requestAnimationFrame(function () {
      requestAnimationFrame(function () {
        orbitProgress.style.transition = 'stroke-dashoffset 1s linear, stroke 0.5s';
      });
    });

    timer = setInterval(function () {
      remaining--;
      updateOrbit();
      if (remaining <= 0) {
        clearInterval(timer);
        timer = null;
        orbit.classList.remove('active');
        secondsBadge.classList.remove('active', 'warn', 'danger');
        if (onExpired) onExpired();
      }
    }, 1000);
  }

  function resetCountdown(seconds) {
    if (timer == null) return;
    clearInterval(timer);
    startCountdown(seconds || total);
  }

  function stopCountdown() {
    clearInterval(timer);
    timer = null;
    orbit.classList.remove('active');
    secondsBadge.classList.remove('active', 'warn', 'danger');
    orbitProgress.classList.remove('warn', 'danger');
  }

  function isRunning() { return timer != null; }

  window.yetiCountdown = {
    startCountdown: startCountdown,
    resetCountdown: resetCountdown,
    stopCountdown: stopCountdown,
    isRunning: isRunning,
    set onCountdownExpired(fn) { onExpired = fn; },
    get onCountdownExpired() { return onExpired; }
  };
})();
