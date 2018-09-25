var srcString = "";

function md5(src) {
    if (typeof(src) !== 'string') {
        return null;
    }
    var step1Res = addTail(src);
    function addTail(src) {
        var len = src.length;
        var mod = len %
    }
}

var destString = md5(srcString);
console.log("the result is ", destString);