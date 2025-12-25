/* luflow.net web site */
/* Public domain 2025. All rights waived */

// handle menu toggle while in mobile (or smaller) type screens:
let menuToggle = document.querySelector('.menuToggle');
let header = document.querySelector('header');
menuToggle.onclick = function () {
    header.classList.toggle('active');
};

// hide header navigation bar when scrolling down, and show again when scrolling up:
let lastScrollTop = 0;
let navbarHeight = 100;
let didScroll = false;
let delta = 5;

window.onscroll = function () {
    didScroll = true;
};

setInterval(function () {
    if (didScroll) {
        hasScrolled();
        didScroll = false;
    }
}, 250);

function hasScrolled() {
    let scrollTop = document.documentElement.scrollTop;

    if (Math.abs(lastScrollTop - scrollTop) <= delta) {
        return;
    }

    if (scrollTop > lastScrollTop && scrollTop > navbarHeight) {
        header.classList.add('nav-up');
        header.classList.remove('nav-down');
    } else {
        header.classList.remove('nav-up');
        header.classList.add('nav-down');
    }

    lastScrollTop = scrollTop;
}
