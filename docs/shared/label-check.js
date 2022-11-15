/**
 * Checks whether the given label is sane.
 *
 * The check corresponds to the following regular expression:
 *
 * ```text
 * ^[a-z_][a-z0-9_-]{0,29}(\\.[a-z_][a-z0-9_-]{0,29})*$
 * ```
 *
 * Same implementation as `matches_label_regex` in `glean-core/src/metrics/labeled.rs`.
 */
function matchesLabelRegex(value) {
  const MAX_LABEL_LENGTH = 61;
  if (value.length > MAX_LABEL_LENGTH) {
    return false;
  }

  let index = 0;
  while (true) {
    let nextChar = value.charAt(index);
    index++;

    if (nextChar.match(/[_a-z]/)) {
      // pass
    } else {
      return false;
    }

    let count = 0;
    while (true) {
      let nextChar = value.charAt(index);
      index++;

      if (!nextChar) {
        return true;
      } else if (nextChar.match(/[a-z0-9_-]/)) {
        // pass
      } else if (nextChar == '.') {
        break;
      } else {
        return false;
      }

      count++;

      if (count == 29) {
        return false;
      }
    }
  }
}

let span = document.querySelector("#result");
document.querySelector("input#label").addEventListener("keyup", (e) => {
  let value = e.target.value;
  if (matchesLabelRegex(value)) {
    span.innerText = "✅ valid";
  } else {
    span.innerText = "❌ invalid";
  }
});
