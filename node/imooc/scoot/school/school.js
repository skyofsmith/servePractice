var klass = require('./klass');

function add(klasses) {
    klasses.forEach(function(item) {
        var teacherName = item.teacherName;
        var students = item.students;
        klass.add(teacherName, students);
    })
}
exports.add = add;
