import React from 'react';

enum ChangedBy {
    Navigation
    Manually
}

function MyComponent() {
    const navigation = ChangedBy.Navigation;
    console.log("Navigation is ${navigation}")
    return <p>Hello World</p>
}