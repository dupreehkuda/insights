document.addEventListener('DOMContentLoaded', function() {
    const slider = document.querySelector('.slider');
    const prevBtn = document.querySelector('.prev-btn');
    const nextBtn = document.querySelector('.next-btn');
    const cards = document.querySelectorAll('.card');

    let currentIndex = 0;

    function showCard(index) {
        slider.style.transform = `translateX(-${index * 100}%)`;
    }

    function nextCard() {
        if (currentIndex < cards.length - 1) {
            cards[currentIndex].classList.add('flipping');
            currentIndex++;
            showCard(currentIndex);
            setTimeout(() => {
                cards[currentIndex - 1].classList.remove('flipping');
            }, 500);
        }
    }

    function prevCard() {
        if (currentIndex > 0) {
            currentIndex--;
            cards[currentIndex].classList.add('flipping');
            showCard(currentIndex);
            setTimeout(() => {
                cards[currentIndex].classList.remove('flipping');
            }, 500);
        }
    }

    nextBtn.addEventListener('click', nextCard);
    prevBtn.addEventListener('click', prevCard);

    // Swipe functionality for mobile
    let touchStartX = 0;
    let touchEndX = 0;

    slider.addEventListener('touchstart', e => {
        touchStartX = e.changedTouches[0].screenX;
    });

    slider.addEventListener('touchend', e => {
        touchEndX = e.changedTouches[0].screenX;
        handleSwipe();
    });

    function handleSwipe() {
        if (touchStartX - touchEndX > 50) {
            nextCard();
        } else if (touchEndX - touchStartX > 50) {
            prevCard();
        }
    }

    // Initialize the slider
    showCard(0);
});

