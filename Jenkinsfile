// Объявляем переменную для скрипта
def gv

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
            agent {
                docker {
                    image 'rust:latest'
                }
            }
            steps {
                script {
                    gv.cargoBuild()
                }
            }
        }

        stage("Build image") {
            // Запускаем сборку прямо на ноде Jenkins, без обертки в docker:latest
            agent any 
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
                }
            }
        }
    }
}
