pipeline {
    agent any

    stages {
        stage("Build Rust") {
            steps {
                sh '''
                docker run --rm \
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
                    credentialsId: 'docker',
                    passwordVariable: 'PASSWORD',
                    usernameVariable: 'USER'
                )]) {
                    sh '''
                    docker build -t kayorie/learning_docker_rx7:jenkins-pipeline .
                    echo $PASSWORD | podman login -u $USER --password-stdin
                    docker push kayorie/learning_docker_rx7:jenkins-pipeline
                    '''
                }
            }
        }
    }
}
