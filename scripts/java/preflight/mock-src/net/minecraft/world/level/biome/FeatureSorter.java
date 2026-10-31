package net.minecraft.world.level.biome;

import java.util.List;
import java.util.function.ToIntFunction;

/** Mock FeatureSorter + the nested StepFeatureData record. */
public final class FeatureSorter {

    private FeatureSorter() {}

    public static final class StepFeatureData {
        private final List<Object> features;
        private final ToIntFunction<Object> indexMapping;

        public StepFeatureData(List<Object> features) {
            this.features = features;
            java.util.Map<Object, Integer> idx = new java.util.IdentityHashMap<>();
            for (int i = 0; i < features.size(); i++) {
                idx.put(features.get(i), i);
            }
            this.indexMapping = idx::get;
        }

        public List<Object> features() {
            return features;
        }

        public ToIntFunction<Object> indexMapping() {
            return indexMapping;
        }
    }
}
