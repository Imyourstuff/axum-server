def gv

//Comment to test branches
pipeline {
    agent any
    stages {
        //Trying to build cargo from docker agent.
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
                    args '-u root:root'
                }
            }
            steps {
                script {
                    gv.cargoBuild()
                }
            }
        }

        //Building image 
        stage("Build image") {
            agent {
                docker {
                    image 'quay.io/podman/stable:latest'
                    args '-v /run/podman/podman.sock:/var/run/docker.sock'
                }
            }

            environment {
                HOME = "${WORKSPACE}" 
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
                }
            }
        }
    }
}