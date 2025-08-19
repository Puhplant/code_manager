<script lang="ts">
  import { post, setAuthToken, ApiError } from '../utils/request';
  import { goto } from '@mateothegreat/svelte5-router';
  import Button from '../components/Button.svelte';
  import Input from '../components/Input.svelte';
  import Card from '../components/Card.svelte';
  import Toast, { showError } from '../components/Toast.svelte';

  let email = $state('');
  let password = $state('');
  let isLoading = $state(false);
  let fieldErrors = $state<Record<string, string>>({});



  function validateForm(): boolean {
    fieldErrors = {};
    
    if (!email.trim()) {
      fieldErrors.email = 'Email is required';
    } else if (!email.includes('@')) {
      fieldErrors.email = 'Please enter a valid email address';
    }
    
    if (!password.trim()) {
      fieldErrors.password = 'Password is required';
    } else if (password.length < 6) {
      fieldErrors.password = 'Password must be at least 6 characters';
    }
    
    return Object.keys(fieldErrors).length === 0;
  }

  async function handleSubmit(event: Event) {
    event.preventDefault();
    
    if (!validateForm()) {
      return;
    }

    isLoading = true;
    
    try {
      const response = await post('/api/auth/login', {
        email: email.trim(),
        password: password
      });
      
      const expiresAt = new Date(response.data.expires_at);
      setAuthToken(response.data.token, expiresAt);
      
      goto('/');
      
    } catch (error) {
      if (error instanceof ApiError) {
        if (error.badRequest) {
          if (error.badRequest.field_errors) {
            error.badRequest.field_errors.forEach(fieldError => {
              fieldErrors[fieldError.field] = fieldError.message;
            });
          }
          showError(error.badRequest.message);
        } else {
          showError(error.message);
        }
      } else {
        showError('An unexpected error occurred. Please try again.');
      }
    } finally {
      isLoading = false;
    }
  }

  function clearFieldError(field: string) {
    if (fieldErrors[field]) {
      delete fieldErrors[field];
      fieldErrors = { ...fieldErrors };
    }
  }
</script>

<div class="min-h-screen flex flex-col justify-center py-12 sm:px-6 lg:px-8">
  <div class="sm:mx-auto sm:w-full sm:max-w-md">
    <div class="text-center">
      <h2 class="mt-6 text-3xl font-extrabold text-foreground">
        Sign in to your account
      </h2>
    </div>
  </div>

  <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
    <Card class="py-8 px-4 sm:px-10">
      <form class="space-y-6" onsubmit={handleSubmit}>
        <div>
          <Input
            id="email"
            name="email"
            type="email"
            autoComplete="email"
            required
            bind:value={email}
            onInput={() => clearFieldError('email')}
            placeholder="Enter your email"
            label="Email address"
            error={fieldErrors.email}
          />
        </div>

        <div>
          <Input
            id="password"
            name="password"
            type="password"
            autoComplete="current-password"
            required
            bind:value={password}
            onInput={() => clearFieldError('password')}
            placeholder="Enter your password"
            label="Password"
            error={fieldErrors.password}
          />
        </div>

        <div>
          <Button
            type="submit"
            loading={isLoading}
            variant="default"
            size="default"
            class="w-full"
          >
            Sign in
          </Button>
        </div>
      </form>

      <div class="mt-6">
        <div class="relative">
          <div class="absolute inset-0 flex items-center">
            <div class="w-full border-t border-border"></div>
          </div>
          <div class="relative flex justify-center text-sm">
            <span class="px-2 bg-card text-muted-foreground">
              <a href="/forgot-password" class="font-medium text-primary hover:text-primary/80">
                Forgot your password?
              </a>
            </span>
          </div>
        </div>
      </div>
    </Card>
  </div>
</div>

<Toast />
