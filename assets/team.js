document.addEventListener('DOMContentLoaded', function() {
    let teamMembers = document.querySelectorAll('[data-team-member]');

    document.addEventListener('mousemove', function(event) {
        let cursorX = event.pageX;
        let cursorY = event.pageY;

        teamMembers.forEach(function(member) {
            setImageAccordingToMouse(cursorX, cursorY, member);
        });
    });
});

function setImageAccordingToMouse(cursorX, cursorY, pictureElement) {
    if (window.innerWidth <= 640) {
        setImage(pictureElement, 0);
        return
    }

    let offsets = getDocumentOffsets(pictureElement);
    let elementX = offsets.x;

    let elementY = offsets.y;
    let height = pictureElement.offsetHeight;

    let width = pictureElement.offsetWidth;
    let elementCenterX = elementX + width / 2;

    let elementCenterY = elementY + height / 2;
    let yOnImage = cursorY > elementY && cursorY < elementY + height;
    let xOnImage = cursorX > elementX && cursorX < elementX + width;

    let onImage = yOnImage && xOnImage;
    if (onImage) {
        setImage(pictureElement, 0);
        return;
    }

    let angleDeg = (Math.atan2(cursorY - elementCenterY, cursorX - elementCenterX) * 180) / Math.PI;
    let withoutNegatives = angleDeg < 0 ? angleDeg + 360 : angleDeg;

    let corrected = (withoutNegatives + 15) % 360;
    let quotient = Math.floor(corrected / 30);

    let imageDirection = quotient + 1;
    setImage(pictureElement, imageDirection);
}

function getDocumentOffsets(element) {
    let x = 0, y = 0;
    for (; element != null; x += element.offsetLeft, y += element.offsetTop, element = element.offsetParent);
    return { x: x, y: y };

}

function setImage(pictureElement, position) {
    const imageHeight = 200;
    pictureElement.style.backgroundPosition = '0px -' + (position * imageHeight) + 'px';

}
