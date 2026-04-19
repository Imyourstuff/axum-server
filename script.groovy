def cargoBuild() {
    echo 'Build started.'
    sh "cargo build --release"
}

def buildAndPushImage() {
    echo "Building docker image."
    withCredentials([
    usernamePassword(
        credentialsId: 'docker',
        passwordVariable: 'PASSWORD',
        usernameVariable: 'USER'
    )
    ]) {
    sh 'sudo docker build -t kayorie/learning_docker_rx7:jenkins-pipeline .'
    sh 'echo $PASSWORD | docker login -u $USER --password-stdin'
    sh 'sudo docker push kayorie/learning_docker_rx7:jenkins-pipeline'
    }
}

def deployApp() {
    echo "deploy the app version ${params.VERSION}"
}
return this
