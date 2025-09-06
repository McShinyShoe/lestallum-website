<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Support\Facades\Hash;
use Illuminate\Support\Facades\Http;
use App\Models\User;
use App\Models\McNameVerification;
use Carbon\Carbon;

class AuthController extends Controller
{
    public function login(Request $request)
    {
        $request->validate([
            'mc_name' => 'required|string',
            'password' => 'required|string',
        ]);

        $user = User::where('mc_name', $request->mc_name)->first();

        if (!$user || !Hash::check($request->password, $user->password)) {
            return make_response(null, 'Invalid credentials', Response::HTTP_UNAUTHORIZED, false);
        }

        $token = $user->createToken('api-token')->plainTextToken;

        return make_response(['token' => $token], 'Login successful');
    }

    public function logout(Request $request)
    {
        $request->user()->currentAccessToken()->delete();
        return make_response(null, 'Token revoked');
    }

    public function generateVerificationCode(Request $request)
    {
        $user = $request->user();

        if ($user->mc_name_verified_at) {
            return make_response(null, 'Minecraft name already verified', 400, false);
        }

        McNameVerification::where('mc_name', $user->mc_name)->delete();

        $code = random_int(100000, 999999);

        McNameVerification::create([
            'mc_name'    => $user->mc_name,
            'code'       => $code,
            'expires_at' => Carbon::now()->addMinutes(15),
        ]);

        Http::post('http://mineflayer:3000/mail', [
            'player'  => $user->mc_name,
            'message' => "Lestallum website verification code: {$code}",
        ]);

        return make_response(null, 'Verification code sent');
    }

    public function submitVerificationCode(Request $request)
    {
        $request->validate([
            'code' => 'required|numeric',
        ]);

        $user = $request->user();

        if ($user->mc_name_verified_at) {
            return make_response(null, 'Minecraft name already verified', 400, false);
        }

        $verification = McNameVerification::where('mc_name', $user->mc_name)
            ->where('code', $request->code)
            ->first();

        if (!$verification) {
            return make_response(null, 'Invalid verification code', 400, false);
        }

        if ($verification->expires_at->isPast()) {
            $verification->delete();
            return make_response(null, 'Verification code expired', 400, false);
        }

        $user->mc_name_verified_at = Carbon::now();
        $user->save();

        $verification->delete();

        return make_response(null, 'Minecraft name verified successfully');
    }

}
