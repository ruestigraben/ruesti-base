package dev.ruestigraben.sbt

import java.io.File

import sbt._
import sbt.Keys._

import scala.sys.process.Process

object RuestiPlugin extends AutoPlugin {

  object autoImport {
    lazy val ruestiTargetDirectory = settingKey[File]("Rust target directory")
    lazy val ruestiCrateName = settingKey[String]("Crate name")
    lazy val ruestiTargetName = settingKey[String]("Name of the bitcode file")
    lazy val ruestiCargoBuild = taskKey[File]("Run cargo build")
  }

  import autoImport._

  override def projectSettings: Seq[Def.Setting[_]] = Seq(
    ruestiTargetDirectory := target.value / "cargo",
    ruestiCrateName := name.value.replace('-', '_'),
    ruestiTargetName := s"${ruestiCrateName.value}.bc",
    ruestiCargoBuild := {
      val log = streams.value.log

      val dir = baseDirectory.value
      val target = ruestiTargetDirectory.value.getAbsoluteFile
      val name = ruestiCrateName.value
      val env = List("CARGO_TARGET_DIR" -> target.toString)

      def runCargoCmd(args: List[String]) =
        Process("cargo" :: args, dir, env: _*).!

      log.info("Cleaning cargo ...")
      IO.delete(target)

      log.info("Running cargo rustc ...")
      assert(runCargoCmd(List("rustc", "--release")) == 0)

      val files = ((target / "release" / "deps") ** s"${name}-*.bc").get();

      files match {
        case Seq(bitcode) => bitcode
        case _ => sys.error("expected exactly one bitcode file")
      }
    },
    Compile / resourceGenerators += Def.task {
      val in = ruestiCargoBuild.value
      val out = (Compile / resourceManaged).value / ruestiTargetName.value
      IO.copyFile(in, out)
      Seq(out)
    }
  )

}
