import { computed, onBeforeUnmount, onMounted, Ref, ref, watch } from "vue";

export function withCheckableEmit() {
  return {
    grantPoints(payload: { achievedPoints: number; totalPoints: number }) {
      return (
        payload.achievedPoints <= payload.totalPoints ||
        (payload.totalPoints < 0 && payload.achievedPoints <= 0)
      );
    },
  };
}

export function useCheckable(
  trigger: Ref<boolean>,
  emit: (
    event: "grantPoints",
    payload: {
      achievedPoints: number;
      totalPoints: number;
    }
  ) => void,
  check: () => number,
  points?: number
) {
  const checked = ref(false);
  const correct = ref(false);
  const achievedPoints = ref(0);
  const totalPoints = ref(points ?? 1);

  const right = computed(() => checked.value && correct.value);
  const wrong = computed(() => checked.value && !correct.value);

  onMounted(() =>
    emit("grantPoints", {
      achievedPoints: achievedPoints.value,
      totalPoints: totalPoints.value,
    })
  );

  onBeforeUnmount(() =>
    emit("grantPoints", {
      achievedPoints: -achievedPoints.value,
      totalPoints: -totalPoints.value,
    })
  );

  watch(trigger, () => {
    checked.value = true;
    achievedPoints.value = check();
    correct.value = achievedPoints.value === totalPoints.value;
    emit("grantPoints", {
      achievedPoints: achievedPoints.value,
      totalPoints: totalPoints.value,
    });
  });

  return {
    right,
    wrong,
    achievedPoints,
    totalPoints,
  };
}
