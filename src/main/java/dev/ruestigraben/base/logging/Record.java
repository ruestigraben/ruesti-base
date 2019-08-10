package dev.ruestigraben.base.logging;

import org.slf4j.event.Level;

public final class Record {

    public final Level level;
    public final String name;
    public final String message;
    public final String file;
    public final String line;

    public Record(String level, String name, String message, String file, long line) {
        this.level = Level.valueOf(level);
        this.name = name;
        this.message = message;
        this.file = file;
        this.line = line >= 0 ? Long.toString(line) : "";
    }

}
