document.addEventListener('DOMContentLoaded', () => {
  function makeInvalid($el) {
    $el.classList.add('is-danger');
  }

  function makeValid($el) {
    $el.classList.remove('is-danger');
  }

  (document.querySelectorAll('.js-validation') || []).forEach(($input) => {
    $input.addEventListener('invalid', () => {
      makeInvalid($input);
    });
    $input.addEventListener('valid', () => {
      makeValid($input);
    });
  });
});