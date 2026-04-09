def buildApp() {
    echo 'building the app'
}

def testApp() {
    echo 'test the app'
}

def deployApp() {
    echo "deploy the app version ${params.VERSION}"
}
return this