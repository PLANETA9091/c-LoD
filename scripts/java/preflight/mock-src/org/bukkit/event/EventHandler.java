package org.bukkit.event;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

// Mock of org.bukkit.event.EventHandler — same members + defaults as the
// real annotation so the runtime scan semantics match.
@Target(ElementType.METHOD)
@Retention(RetentionPolicy.RUNTIME)
public @interface EventHandler {
    EventPriority priority() default EventPriority.NORMAL;

    boolean ignoreCancelled() default false;
}
