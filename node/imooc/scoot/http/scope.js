var globalVariable = 'This is global variable';

function globalFunction() {
    var localVariable = 'This is global variable';
    console.log('Visit global/local variable');
    console.log(globalVariable);
    console.log(localVariable);
}

globalFunction();
