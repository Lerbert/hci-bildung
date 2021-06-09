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
      Alternativ können Sie folgenden Link verwenden:
      <div class="is-flex">
        <input id="link" class="input" type="text" value="https://4m6.de/r" readonly>
        <button class="button" v-on:click="copy">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" width="20" height="20" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
          </svg>
          {{ copyText }}
        </button>
      </div>
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

  data() {
    return {
      copyText: "Link kopieren",
      copyTimeout: undefined,
    }
  },

  methods: {
    showShare: function () {
      var modal = document.getElementById("shareModal");
      modal.style.display = "block";
    },

    closeShare: function () {
      var modal = document.getElementById("shareModal");
      modal.style.display = "none";
    },

    copy() {
      if (this.copyTimeout) {
        clearTimeout(this.copyTimeout);
      }

      const copyText = document.getElementById("link");

      copyText.select();
      copyText.setSelectionRange(0, 99999);

      document.execCommand("copy");

      this.copyText = "Link kopiert!";
      this.copyTimeout = setTimeout(() => this.copyText = "Link kopieren", 3000);
    },
  }
}
</script>

<style lang="scss" scoped>

.modal {
  display: none; /* Hidden by default */
  position: fixed; /* Stay in place */
  z-index: 9001; /* Sit on top */
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