pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }
    stages {
        //Trying to build cargo from docker agent.
        stage("Cargo build Rust") {
            steps {
                script {
                    echo "Build started..."
                    sh "cargo build --release"
                }
            }
        }
        //Building image 
        stage("Build image") {
            steps {
                script {
                    echo "Building docker image."
                    withCredentials([
                        usernamePassword(
                            credentialsId: 'docker-hub-repo',
                            passwordVariable: 'PASSWORD',
                            usernameVariable: 'USER'
                        )
                    ])
                    sh 'docker build -t kayorie/learning_docker_rx7:jenkins-pipeline .'
                    sh 'echo $PASSWORD | docker login -u $USERNAME --password-stdin'
                    sh 'docker push kayorie/learning_docker_rx7:jenkins-pipeline'
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