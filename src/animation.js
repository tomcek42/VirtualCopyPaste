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
    getCoord();
  }

  function onEmailFocus() { getCoord(); }
  function onEmailBlur() { resetFace(); }

  function coverEyes() {
    gsap.to(armL, { duration: .45, x: -93, y: 2, rotation: 0, ease: 'quad.out' });
    gsap.to(armR, { duration: .45, x: -93, y: 2, rotation: 0, ease: 'quad.out', delay: .1 });
  }

  function uncoverEyes() {
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

  window.yetiAnimation = { coverEyes: coverEyes, uncoverEyes: uncoverEyes, resetFace: resetFace };
})();
