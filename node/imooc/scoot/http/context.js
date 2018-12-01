
var pet = {
    words: '...',
    speak: function() {
        console.log(this.words);
        console.log(this === pet);
    }
};

pet.speak();


function ppet(words) {
    this.words = words;

    console.log(this.words);
    console.log(this === global);
}

ppet('...');

function Pet(words) {
    this.words = words;
    this.speak = function() {
        console.log(this.words);
        console.log(this);
    };
}
var cat = new Pet('Miao');
cat.speak();
