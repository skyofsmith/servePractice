var EventEmitter = require('events').EventEmitter;

var life = new EventEmitter();

life.setMaxListeners(11);
function water(who) {
    console.log('to ' + who + ' drink');
}
life.on('anwei', water);
life.on('anwei', function(who) {
    console.log('to ' + who + ' eat');
});
life.on('anwei', function(who) {
    console.log('to ' + who + ' 3');
});
life.on('anwei', function(who) {
    console.log('to ' + who + ' 4');
});
life.on('anwei', function(who) {
    console.log('to ' + who + ' 5');
});
life.on('anwei', function(who) {
    console.log('to ' + who + ' 6');
});
life.on('anwei', function(who) {
    console.log('to ' + who + ' 7');
});
life.on('anwei', function(who) {
    console.log('to ' + who + ' 8');
});
life.on('anwei', function(who) {
    console.log('to ' + who + ' 9');
});
life.on('anwei', function(who) {
    console.log('to ' + who + ' 10');
});
life.on('anwei', function(who) {
    console.log('to ' + who + ' too many!!!');
});
life.on('niai', function(who) {
    console.log('to ' + who + ' buy clothes');
});
life.on('niai', function(who) {
    console.log('to ' + who + ' give crash');
});

life.removeListener('anwei', water);
life.removeAllListeners();

var hasConfortListerer = life.emit('anwei', 'sam');
var hasLovedListerer = life.emit('niai', 'girl');
// var hasPlayedListerer = life.emit('wan', 'whatever');

console.log(hasConfortListerer);
console.log(hasLovedListerer);

console.log(life.listeners('anwei').length);
console.log(EventEmitter.listenerCount(life, 'anwei'));
