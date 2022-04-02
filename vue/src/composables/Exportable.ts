import { Ref, watch } from "vue";

export function useExportable<T>(
  trueObject: Ref<T>,
  updateExport: () => void,
  updateOnChangeOf: Ref<unknown>[]
) {
  watch(trueObject, updateExport);
  watch(updateOnChangeOf, updateExport);
}
