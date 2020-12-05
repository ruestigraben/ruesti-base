package dev.ruestigraben.base.test

import dev.ruestigraben.base.Logging
import org.graalvm.polyglot.{Context, Source}
import munit.FunSuite

class BaseTests extends FunSuite {

  test("Integration") {
    val context = Context.newBuilder().allowAllAccess(true).build()

    val resource = getClass.getClassLoader.getResource("ruesti_base_test.bc")
    val source = Source.newBuilder("llvm", resource).build()
    context.eval(source)

    Logging.install(context)

    context.getBindings("llvm").getMember("__test").as(classOf[Runnable]).run()
  }

}
