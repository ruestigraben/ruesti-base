package dev.ruestigraben.sbt

import java.io.File

import sbt._
import sbt.Keys._

import scala.sys.process.Process

object RuestiPlugin extends AutoPlugin {

  val Cargo = Tags.Tag("Cargo")

  object autoImport {
    lazy val ruestiTargetDirectory = settingKey[File]("Rust target directory")
    lazy val ruestiCrateName = settingKey[String]("Crate name")
    lazy val ruestiTargetName = settingKey[String]("Name of the bitcode file")
    lazy val ruestiCargoBuild = taskKey[File]("Run cargo build")
    lazy val ruestiFeatures = settingKey[List[String]]("Feature flags to build")
  }

  import autoImport._

  val cargoBuildTask: Def.Initialize[Task[File]] = Def.task {
    val log = streams.value.log

    val dir = baseDirectory.value
    val target = ruestiTargetDirectory.value.getAbsoluteFile
    val name = ruestiCrateName.value
    val env = List(
      "CARGO_TARGET_DIR" -> target.toString,
      // <https://github.com/dtolnay/cargo-llvm-lines/issues/4>
      "CARGO_INCREMENTAL" -> ""
    )
    val features = ruestiFeatures.value match {
      case Nil => Nil
      case features => List("--features", features.mkString(" "))
    }

    def runCargoCmd(args: List[String]) =
      Process("cargo" :: args, dir, env: _*).!

    log.info(s"Compiling Rust sources with features ${ruestiFeatures.value.mkString("[", ",", "]")} to ${target} ...")
    assert(runCargoCmd("rustc" :: "--release" :: features) == 0)

    val files = ((target / "release" / "deps") ** s"${name}-*.bc").get()

    files match {
      case Seq(bitcode) => bitcode.asFile
      case _ => sys.error("expected exactly one bitcode file")
    }
  } tag(Cargo)

  val configSettings: Seq[Def.Setting[_]] = Seq(
    resourceGenerators += Def.task {
      val log = streams.value.log

      val in = ruestiCargoBuild.value
      val out = resourceManaged.value / ruestiTargetName.value
      log.info(s"Copying $in to $out")
      IO.copyFile(in, out)
      Seq(out)
    },
    ruestiCargoBuild := cargoBuildTask.value
  )

  override def projectSettings: Seq[Def.Setting[_]] = Seq(
    Compile / ruestiTargetDirectory := target.value / "cargo",
    Test / ruestiTargetDirectory := target.value / "cargo_test",
    ruestiCrateName := name.value.replace('-', '_'),
    Compile / ruestiTargetName := s"${ruestiCrateName.value}.bc",
    Test / ruestiTargetName := s"${ruestiCrateName.value}_test.bc",
    Compile / ruestiFeatures := Nil,
    Test / ruestiFeatures := List("ruesti_test"),
    concurrentRestrictions += Tags.limit(Cargo, 1)
  ) ++ inConfig(Compile)(configSettings) ++ inConfig(Test)(configSettings)

}
