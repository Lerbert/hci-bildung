import { ref, Ref, watch } from "vue";
import debounce from "lodash/debounce";

import { SaveStatus } from "../enums";

export function useSaveable<T>(
  url: string,
  autosave: Ref<boolean>,
  entity: Ref<T>,
  validate: (entity: T) => boolean
) {
  const saveStatus = ref(
    autosave.value ? SaveStatus.SAVED : SaveStatus.DISABLED
  );
  const saveBackoff = ref(0);
  const save = debounce(saveHelper, 1000);
  async function saveHelper() {
    if (!validate(entity.value)) {
      saveStatus.value = SaveStatus.FAILED;
      return;
    }
    saveStatus.value = SaveStatus.SAVING;
    const delay = new Promise((r) => setTimeout(r, 500)); // Delay to show user that we are indeed saving
    let saveOk = false;
    try {
      const response = await fetch(url, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(entity.value),
      });
      await delay;
      saveOk = response.status === 200;
    } catch (e) {
      console.log("Error while saving:", e);
      saveOk = false;
    }
    if (saveStatus.value === SaveStatus.SAVING) {
      if (saveOk) {
        saveStatus.value = SaveStatus.SAVED;
        saveBackoff.value = 0;
      } else if (saveBackoff.value >= 5000) {
        saveStatus.value = SaveStatus.FAILED;
        console.log("Saving failed. No further automatic retries");
      } else {
        saveBackoff.value += 1000;
        console.log(
          "Saving failed. Retrying automatically in",
          saveBackoff.value / 1000,
          "s"
        );
        setTimeout(save, saveBackoff.value);
      }
    }
  }

  function handleEntityChange() {
    if (autosave.value) {
      saveStatus.value = SaveStatus.WAITING;
      save();
    }
  }
  watch(entity, handleEntityChange, { deep: true });

  return { save, saveStatus };
}
