<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::table('users', function (Blueprint $table) {
            $table->renameColumn('email', 'mc_name');
            $table->renameColumn('email_verified_at', 'mc_name_verified_at');
        });

        // Rename column in password_reset_tokens table
        Schema::table('password_reset_tokens', function (Blueprint $table) {
            $table->renameColumn('email', 'mc_name');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::table('users', function (Blueprint $table) {
            $table->renameColumn('mc_name', 'email');
            $table->renameColumn('mc_name_verified_at', 'email_verified_at');
        });

        Schema::table('password_reset_tokens', function (Blueprint $table) {
            $table->renameColumn('mc_name', 'email');
        });
    }
};
