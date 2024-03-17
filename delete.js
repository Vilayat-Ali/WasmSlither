const fs = require("node:fs");

(function () {
  fs.unlink("./web/static/.gitignore", function (err) {
    if (err) {
      console.error(err);
      process.exit(1);
    }
  });
})();
