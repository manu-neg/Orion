async function handleLogin(event) {    
    event.preventDefault();
    const form = event.target;
    const username = form.username.value;
    const password = form.password.value;
    
    let error_box = document.getElementById('error-message');
    if (error_box) {
        error_box.textContent = '';
    }
    try {
        const response = await fetch('/api/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, password })
        });

        if (response.ok) {
            const data = await response.json();
            localStorage.setItem('token', data.token);
            document.cookie = `authToken=${data.token}; path=/; secure; samesite=strict; max-age=86400`;  // 24 hours
            window.location.href = '/';
        } else {
            let errMsg = await response.json();
            throw new Error(errMsg.error || 'Login failed');
        }

    } catch (error) {
        if (error_box) {
            error_box.textContent = error.message;
        }
        console.error('Login failed:', error);
    }

}