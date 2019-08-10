package dev.ruestigraben.base.test

import dev.ruestigraben.base.Logging
import org.graalvm.polyglot.{Context, Source}
import utest._

object BaseTests extends TestSuite {

  val tests = Tests {
    test("Integration") {
      val context = Context.newBuilder().allowAllAccess(true).build();

      val resource = getClass.getClassLoader.getResource("ruesti_base.bc")
      val source = Source.newBuilder("llvm", resource).build()
      context.eval(source)

      Logging.install(context)

      context.getBindings("llvm").getMember("__test").as(classOf[Runnable]).run()
    }
  }

}
