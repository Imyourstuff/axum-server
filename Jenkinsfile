pipeline {
    agent any

    stages {
        stage("Initialize Groovy script") {
            steps {
                script {
                    gv = load "script.groovy"
                }
            }
        }

        stage("Cargo build Rust") {
            steps {
                script {
                    gv.cargoBuild()
                }
            }
        }

        stage("Build image") {
            environment {
                HOME = "${WORKSPACE}"   // необходимо для rootless Podman
            }
            steps {
                script {
                    gv.buildAndPushImage()
                }
            }
        }

        stage("Deploy!") {
            steps {
                script {
                    echo "Deploying the app!"
                    // при желании: gv.deployApp()
                }
            }
        }
    }
}