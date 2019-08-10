package dev.ruestigraben.base;

import dev.ruestigraben.base.logging.Record;
import org.graalvm.polyglot.Context;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.slf4j.MDC;

import java.util.function.Consumer;

public class Logging implements Consumer<Record> {

    private static final Logger logger = LoggerFactory.getLogger(Logging.class);

    private static final Logging INSTANCE = new Logging();

    private Logging() {}

    @Override
    public void accept(Record record) {
        Logger logger = LoggerFactory.getLogger(record.name);
        if (!record.file.isEmpty())
            MDC.put("file", record.file);
        if (!record.line.isEmpty())
            MDC.put("line", record.line);
        logger.info("{}", record.message);
        MDC.remove("file");
        MDC.remove("line");
    }

    public static void install(Context context) {
        logger.info("Installing polyglot logger");
        context.getPolyglotBindings().putMember("__base_logging", INSTANCE);
        context.getBindings("llvm").getMember("__base_init_logger").as(Runnable.class).run();
    }

}

