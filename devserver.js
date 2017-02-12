const express = require("express");

const app = express();
app.use(express.static('build/site'));
app.listen(8080, () => {
  console.info("Development server listening at port 8080");
});
