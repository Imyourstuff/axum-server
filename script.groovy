def cargoBuild() {
    echo 'Build started.'
    // Запускаем сборку внутри контейнера rust:latest с помощью podman
    sh """
        sudo podman run --rm \
            -v ${WORKSPACE}:${WORKSPACE}:Z \
            -w ${WORKSPACE} \
            rust:latest \
            cargo build --release
    """
}

def buildAndPushImage() {
    echo "Building container image with podman."
    withCredentials([
        usernamePassword(
            credentialsId: 'docker-hub-repo',
            passwordVariable: 'PASSWORD',
            usernameVariable: 'USER'
        )
    ]) {
        // Сборка образа через podman
        sh 'podman build -t kayorie/learning_docker_rx7:jenkins-pipeline .'
        // Логин в Docker Hub (podman поддерживает --password-stdin)
        sh 'echo $PASSWORD | podman login -u $USER --password-stdin'
        // Публикация образа
        sh 'podman push kayorie/learning_docker_rx7:jenkins-pipeline'
    }
}

def deployApp() {
    echo "deploy the app version ${params.VERSION}"
}
return this