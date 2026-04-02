def buildApp() {
    echo 'building the app'
}

def testApp() {
    echo 'test the app'
}

def deployApp() {
    echo "deploy the app versiob ${params.VERSION}"
}
return this