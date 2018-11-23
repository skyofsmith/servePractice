var student = require('./student');
var teacher = require('./teacher');

teacher.add('teacher1')

function add(teacherName, students) {
    teacher.add(teacherName);

    students.forEach(function(item, index) {
        student.add(item);
    });
}
module.exports = add;
