
var c = 0;

function printIt() {
    console.log(c);
}

function plus() {
    c += 1;
}

plus()
printIt();

function plusAsync() {
    setTimeout(function() {
        c += 1;
    }, 1000);
}

plusAsync()
printIt();

function plusCallback(callback) {
    setTimeout(function() {
        c += 1;
        callback();
    }, 1000);
}

plusCallback(printIt);
