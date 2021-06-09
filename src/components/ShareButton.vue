<template>
  <button
    class="button is-link"
    v-on:click="showShare"
    :class="{ 'is-active': isActive ? isActive(): null }"
  >
    Mit SuS teilen
  </button>

  <div id="shareModal" class="modal">
    <div class="modal-content">
      <span class="close" v-on:click="closeShare">&times;</span>
      <p>Benutzen Sie den folgenden QR-Code, um Ihr Lernmaterial mit den SuS zu teilen: </p>
      <img src="../assets/qr.png" alt="QR-Code" class="center">
      <p>
        Alternativ können Sie folgenden Link verwenden:
        <a href="https://4m6.de/r">https://4m6.de/r</a>
      </p>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    isActive: {
      type: Function,
      default: null,
    },
  },
  methods: {
    showShare: function () {
      var modal = document.getElementById("shareModal");
      modal.style.display = "block";
    },

    closeShare: function () {
      var modal = document.getElementById("shareModal");
      modal.style.display = "none";
    }
  }
}
</script>

<style lang="scss" scoped>

.modal {
  display: none; /* Hidden by default */
  position: fixed; /* Stay in place */
  z-index: 1; /* Sit on top */
  left: 0;
  top: 0;
  width: 100%; /* Full width */
  height: 100%; /* Full height */
  overflow: auto; /* Enable scroll if needed */
  background-color: rgb(0,0,0); /* Fallback color */
  background-color: rgba(0,0,0,0.4); /* Black w/ opacity */
}

/* Modal Content/Box */
.modal-content {
  background-color: #fefefe;
  margin: 15% auto;
  padding: 20px;
  border: 1px solid #888;
  width: 80%;
}

/* The Close Button */
.close {
  color: #aaa;
  float: right;
  font-size: 28px;
  font-weight: bold;
}

.close:hover,
.close:focus {
  color: black;
  text-decoration: none;
  cursor: pointer;
}

.center {
  display: block;
  margin-left: auto;
  margin-right: auto;
}
</style>