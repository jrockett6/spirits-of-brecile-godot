ThisBuild / scalaVersion := "3.1.2"
ThisBuild / version := "0.1"

// val circeVersion   = "0.14.1"
// val fs2Version     = "3.2.11"
// val munitCEVersion = "1.0.7"
// val http4sVersion = "1.0.0-M35"
enablePlugins(ScalaJSPlugin)
scalaJSUseMainModuleInitializer := true

lazy val commonSettings = Seq(
  semanticdbEnabled := true,
  semanticdbVersion := scalafixSemanticdb.revision
)

// Main build
lazy val client = (project in file("client"))
  .settings(
    name := "spirits-of-brecile",
    organization := "wafflepop.inc",
    run / fork := true,
    commonSettings,

    // Dependencies
    libraryDependencies ++= Seq(
      //   "io.circe"    %% "circe-core"    % circeVersion,
      //   "io.circe"    %% "circe-generic" % circeVersion,
      //   "io.circe"    %% "circe-parser"  % circeVersion,
      //   "co.fs2"      %% "fs2-core"      % fs2Version,
      //   "co.fs2"      %% "fs2-io"        % fs2Version,
      //   "org.typelevel" %% "munit-cats-effect-3" % munitCEVersion % Test
      // "org.http4s"  %% "http4s-client"  % http4sVersion
      // "org.http4s"    %% "http4s-ember-client"    % http4sVersion,
      // "org.http4s"    %% "http4s-jdk-http-client" % httpClientVersion,
      // "org.http4s"    %% "http4s-dsl"             % http4sVersion,
    )
  )
