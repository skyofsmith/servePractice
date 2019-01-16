var express = require('express');
var router = express.Router();

/* GET home page. */
router.get('/', function(req, res, next) {
  res.render('index', { title: 'Express', person: 'nobody' });
});
router.get('/a', function(req, res, next) {
  res.render('index', { title: 'Express', person: 'person a' });
});

module.exports = router;
