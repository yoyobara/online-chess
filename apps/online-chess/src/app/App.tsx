import { Route, Routes } from 'react-router-dom';
import { GuestOnly } from '../components/Routers/GuestOnly';
import { LoggedInOnly } from '../components/Routers/LoggedInOnly';
import { LandingPage } from './landing_page/LandingPage';
import { SignInModal } from './auth_modals/SignInModal';
import { SignUpModal } from './auth_modals/SignUpModal';
import { HomePage } from './home_page/HomePage';
import { PlayPageContainer } from './play_page/PlayPageContainer';
import { PlayPageRealtimeProvider } from './play_page/PlayPageRealtimeProvider';

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
        path="/play/:match_id"
        element={
          <LoggedInOnly>
            <PlayPageRealtimeProvider>
              <PlayPageContainer />
            </PlayPageRealtimeProvider>
          </LoggedInOnly>
        }
      />
    </Routes>
  );
}
