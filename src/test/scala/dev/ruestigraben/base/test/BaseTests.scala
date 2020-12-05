package dev.ruestigraben.base.test

import java.util.ArrayList
import java.util.function.{BiConsumer, Consumer}

import org.graalvm.polyglot.{Context, PolyglotException, Source}
import munit.FunSuite

class BaseTests extends FunSuite {

  object Fixtures {

    var consumed1: Option[String] = None
    val consumer1: Consumer[String] = str => {
      require(consumed1.isEmpty)
      consumed1 = Some(str)
    }

    var consumed2: Option[(String, Int)] = None
    val consumer2: BiConsumer[String, Int] = (str, int) => {
      require(consumed2.isEmpty)
      consumed2 = Some((str, int))
    }

    var ran = false
    val run: Runnable = () => {
      require(!ran)
      ran = true
    }

  }

  test("Integration (success)") {
    val context = Context.newBuilder().allowAllAccess(true).build()

    val resource = getClass.getClassLoader.getResource(BuildInfo.test_ruestiTargetName)
    val source = Source.newBuilder("llvm", resource).build()
    context.eval(source)

    context.getPolyglotBindings.putMember("__test_consumer1", Fixtures.consumer1)
    context.getPolyglotBindings.putMember("__test_consumer2", Fixtures.consumer2)
    context.getPolyglotBindings.putMember("__test_runnable", Fixtures.run)
    context.getPolyglotBindings.putMember("__test_string", "foo")

    context.getBindings("llvm").getMember("__test_success").as(classOf[Runnable]).run()

    assertEquals(Fixtures.consumed1, Some("Hello from Rust"))
    assertEquals(Fixtures.consumed2, Some(("Hello again", 3)))
    assertEquals(Fixtures.ran, true)

    val list1 = context.getPolyglotBindings.getMember("__response_array1")
      .as(classOf[ArrayList[_]])
      .asInstanceOf[ArrayList[AnyRef]]
    assertEquals(list1.size, 1)
    assertEquals(list1.get(0), 2: Integer)

    val list2 = context.getPolyglotBindings.getMember("__response_array2")
      .as(classOf[ArrayList[_]])
      .asInstanceOf[ArrayList[AnyRef]]
    assertEquals(list2.size, 2)
    assertEquals(list2.get(0), 12: Integer)
    assertEquals(list2.get(1), "yolo")
  }

  test("Integration (failure)") {
    val context = Context.newBuilder().allowAllAccess(true).build()

    val resource = getClass.getClassLoader.getResource(BuildInfo.test_ruestiTargetName)
    val source = Source.newBuilder("llvm", resource).build()
    context.eval(source)

    val fn = context.getBindings("llvm").getMember("__test_failure").as(classOf[Runnable])

    try {
      fn.run()
      sys.error("Succeeded unexpectedly")
    }
    catch {
      case ex: PolyglotException =>
        val first = ex.getPolyglotStackTrace.iterator().next()
        if (!first.toString.matches(".*__rust_start_panic.*"))
          sys.error("Unexpected trace")
    }
  }

}
