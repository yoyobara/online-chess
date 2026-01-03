import { Route, Routes } from 'react-router-dom';
import { GuestOnly } from '../components/Routers/GuestOnly';
import { LoggedInOnly } from '../components/Routers/LoggedInOnly';
import { LandingPage } from './landing_page/LandingPage';
import { SignInModal } from './auth_modals/SignInModal';
import { SignUpModal } from './auth_modals/SignUpModal';
import { PlayPage } from './play_page/PlayPage';
import { HomePage } from './home_page/HomePage';

export function App() {
  return (
    <Routes>
      <Route
        path="/"
        element={
          <GuestOnly>
            <LandingPage />
          </GuestOnly>
        }
      />
      <Route
        path="/sign_in"
        element={
          <GuestOnly>
            <SignInModal />
          </GuestOnly>
        }
      />
      <Route
        path="/sign_up"
        element={
          <GuestOnly>
            <SignUpModal />
          </GuestOnly>
        }
      />
      <Route
        path="/home"
        element={
          <LoggedInOnly>
            <HomePage />
          </LoggedInOnly>
        }
      />
      <Route
        path="/play"
        element={
          <LoggedInOnly>
            <PlayPage />
          </LoggedInOnly>
        }
      />
    </Routes>
  );
}
