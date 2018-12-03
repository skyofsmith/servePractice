var EventEmitter = require('events').EventEmitter;

var life = new EventEmitter();

life.on('anwei', function(who){
    console.log('to ' + who + ' drink');
});

life.on('anwei', function(who) {
    console.log('to ' + who + ' eat');
})
life.emit('anwei', 'sam');
