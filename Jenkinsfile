pipeline {
    agent any

    stages {
        stage("Build Rust") {
            steps {
                sh '''
                podman run --rm \
                  -v $PWD:/app \
                  -w /app \
                  rust:latest \
                  cargo build --release
                '''
            }
        }

        stage("Build image") {
            steps {
                withCredentials([usernamePassword(
                    credentialsId: 'docker-hub-repo',
                    passwordVariable: 'PASSWORD',
                    usernameVariable: 'USER'
                )]) {
                    sh '''
                    podman build -t kayorie/learning_docker_rx7:jenkins-pipeline .
                    echo $PASSWORD | podman login -u $USER --password-stdin
                    podman push kayorie/learning_docker_rx7:jenkins-pipeline
                    '''
                }
            }
        }
    }
}
